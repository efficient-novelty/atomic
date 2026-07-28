# Law V2 Genesis Scheme Calculus V1

**Status:** `PROPOSED_NOT_ADOPTED`

**Proposal schema:** `law-v2-gsc-v1-architecture`

**Authority:** none until a versioned adoption binds the rule compiler,
normalizer, quotient checker, and theorem certificates described below.
Current production code must continue to return `Unknown`.

**Adoption readiness:** blocked pending the exact finite code, matcher,
compiler-equation, canonical-ordering, normalization, quotient, and
derivability manifests required by Section 3.5. This document freezes an
architecture and rule schema; it is not yet an installable normative
calculus.

## 1. Purpose and scope

This proposal defines an architecture and rule schema for the previously
missing history-demand operator without identifying it with a search
implementation or with a supplied registry. It fixes:

- the distinction between prospective demand and candidate-local sealing
  trace;
- an anonymous public-interface vocabulary;
- a ranked inductive demand grammar;
- exact seeds, support jurisdiction, and closure order;
- family identity and specialization;
- normalization and a finite witness-generated quotient;
- the manifest-indexed anchored definition \(C^{\mathcal M}_{B_H}(A)\), whose
  live instance is \(C^{\mathcal M}_{B_H}(W_H)\); and
- the induction principles required for total specialization, weakening,
  locality, and expiration.

The proposal preserves the adopted future-hole, dependent-context,
sequential-substitution, opaque-dereference, support-comprehension, and
zero-charge disciplines. If adopted, it replaces the unspecified
scheme-generation relation and the arbitrary-term carrier construction. It
does not change kernel truth conditions, candidate acceptance, or free
sealing. Its implementation may require conservative kernel-facing syntax
capabilities or a trusted GF2 artifact translator that current code does not
yet provide.

## 2. Two different judgments

Let \(H\) be a cumulative sealed history with verified public library \(B_H\).
An ordered anchor pair is

\[
  A=(S^+,S^-)
\]

of verified events in \(H\), where \(S^-\) immediately precedes \(S^+\).
The pair need not still be active. Write \(W_H\) for the unique active anchor
pair at the tip of \(H\). Demand is issued only from \(W_H\); arbitrary
verified anchors are retained so old derivations have a well-typed weakening
target.

For an anchor \(A\), \(B_H^{\leq A}\) is the prefix of the verified boundary
through \(S^+\). Declarations sealed after \(S^+\) are never visible when an
old anchored derivation is replayed.

This proposal uses two disjoint judgments.

The index \(\mathcal M\) denotes the installation manifest specified in
Section 3.5. Before such a manifest is fixed, the following is a judgment
schema rather than one executable relation.

### 2.1 History-generated demand

\[
  B_H;A\vdash_{\mathrm{hist},\mathcal M} d:F
\]

means that the verified anchored prefix generates canonical demand family
\(F\), represented by derivation \(d\). The complete family type is defined
in Section 4. No candidate occurs in this judgment. A live census uses
\(A=W_H\).

### 2.2 Candidate-local sealing trace

\[
  B_H;x\vdash_{\mathrm{seal}}\tau
\]

means that candidate \(x\) carries a structural trace sufficient to integrate
its new payload against \(H\). The trace grammar has the three roles

\[
  \mathsf{Act},\qquad \mathsf{Cmp},\qquad \mathsf{Horn}.
\]

These names import only the shape of the old sealed-extension grammar:

- `Act` records action on one prior public site;
- `Cmp` records compatibility between two public routes; and
- `Horn` records a higher missing-face-plus-filler problem assembled from
  lower public trace.

`Horn` remains part of the exact candidate-local realization. It may be
classified as derived only after a checked translation supplies the public
lower boundary, the open-box decoding, the replacement term, its typing and
boundary equations, allowed-dependency evidence, substitution stability, and
normalization compatibility. The old artifact supplies useful interfaces for
this theorem target; it is not authority for this judgment in GF2.

There is no rule

\[
  B_H;x\vdash_{\mathrm{seal}}\tau
  \Longrightarrow
  B_H;A\vdash_{\mathrm{hist},\mathcal M}d:F.
\]

A candidate may discharge a history demand and must independently satisfy its
sealing trace. Candidate-local trace never creates the demand it is used to
answer.

## 3. Canonical public-interface input

### 3.1 Opaque identifiers

All declarations and events are addressed by canonical digests. Human names,
source ordering unrelated to dependency, and decoder metadata are unavailable
to the generator.

### 3.2 Normalized sealed events

A normalized sealed event contains:

1. a finite dependent public telescope;
2. its finite public equation set;
3. opaque references to earlier declaration types, used whole;
4. canonical dependency edges; and
5. verified structural descriptors computed from the normalized telescope.

Structural descriptors are compiler outputs, not caller role annotations.
Their compiler and its source digest are part of the GSC version.

### 3.3 Descriptor grammar

The descriptor compiler may emit only the following records.

```text
ClosedFormerFrame {
    owner,
    parameter_context,
    constructor_code,
    complete_constructor_certificate,
    recursive_position_masks,
    computation_mode,
    source_declarations
}

IntroPort {
    owner,
    constructor,
    argument_context,
    result_indices,
    boundary_ports,
    source_declarations
}

CellPort {
    owner,
    cell,
    dimension,
    boundary,
    source_declarations
}

GlobalOperationFrame {
    operation,
    parameter_context,
    public_output_role,
    operation_shape,
    footprint_code,
    action_code,
    compatibility_completeness_certificate,
    source_declarations
}

ComparisonProblemFrame {
    source_cell,
    parameter_context,
    orientation,
    left_route,
    right_route,
    designated_missing_output,
    source_declarations
}

MateProblemFrame {
    left_operation,
    right_operation,
    adjunction_data,
    source_cell,
    parameter_context,
    orientation,
    left_route,
    right_route,
    designated_missing_output,
    source_declarations
}
```

A `ClosedFormerFrame` is emitted only for a declaration group carrying a
kernel-checked closed-constructor-diagram token. Its `IntroPort` and `CellPort`
records are exactly the ports of that token. The completeness certificate
states that the listed ports are the complete public constructor boundary for
the admitted strictly-positive code. Merely observing a type and some terms
of that type is insufficient. The recursive-position masks and computation
mode are computed from the checked constructor code, never chosen after a
goal is generated.

`OperationShape`, `FootprintCode`, and `ActionCode` belong to a finite
versioned grammar. Together they give the operation's dependent inputs,
result template, admissible public-site kinds, reindexing maps, and the
syntax-directed action output. The compatibility certificate proves that the
code accepts exactly the sites supported by that operation token; a bare
function type does not determine any of these roles.

`public_output_role` is exactly one of `GlobalOperationPort`,
`UsePort { owner_former_id }`, or `ActionPort { operation_id, site_id,
typed_routes }`. In particular, every `UsePort` identifies the former whose
use interface it realizes.

A `CellPort` is emitted for a public path or cell whose complete boundary is
available from the sealed interface. A `ComparisonProblemFrame` or
`MateProblemFrame` already contains the typed source cell, orientation,
displayed routes, and designated missing output. Unit/counit data or two
parallel-looking terms alone do not select a comparison or mate problem.

Every declaration group in \(B_H^{\leq A}\) receives either a
`CertifiedCannotBecomeRelevant` prefilter proof or exactly one descriptor
extraction result:

```text
Emitted(finite_descriptor_set)
CertifiedIrrelevant
OutsideFragment(reason)
```

The descriptor compiler:

- may inspect normalized types, equations, declaration-group tokens, and
  dependency edges;
- may not inspect whether a later demand is derivable;
- may not accept a proposed completion goal as input; and
- returns `OutsideFragment` rather than inventing a descriptor when syntax
  that could bear a registered role is unsupported.

`CertifiedIrrelevant` contains a kernel-checkable proof that none of the
versioned descriptor codes applies. An `OutsideFragment` result for any group
not already covered by `CertifiedCannotBecomeRelevant` prevents a complete
census; it is never silently omitted.
The prefilter is part of the frozen matcher grammar and may inspect only
normalized head forms, dependency edges, and previously verified role
tokens. The certificate accounts for every pre-anchor group, avoiding a
circular "extract only what later proves relevant" policy.

The deterministic projection

```text
public_ports : Descriptor -> FiniteVec<PublicPort>
```

emits the typed formation, introduction, cell, operation, use, and action
ports carried by a descriptor. Thus \(I^{\mathcal M}_{B_H}(A)\) contains both
descriptors and their projected public ports; rank-\(0\) sources are never
implicit.

### 3.4 Finite eligible view

The rule carrier does not range over the whole mutable catalog. For a verified
anchor \(A\), define \(I^{\mathcal M}_{B_H}(A)\) as the least finite relevance
closure in the immutable prefix \(B_H^{\leq A}\) containing:

1. descriptors originating in either anchor event;
2. opaque dependencies of an already eligible descriptor;
3. a standing operation when its checked footprint matcher accepts an
   anchor-connected eligible site;
4. opaque dependencies of such an operation; and
5. a comparison or mate-problem frame when its typed endpoints are eligible
   and its checked relevance path reaches an anchor-connected site.

The relevance construction inspects every standing descriptor born no later
than \(S^+\) and records a finite derivation for every inclusion. Its matcher
returns `Applicable`, `CertifiedInapplicable`, or `OutsideFragment`. Thus an
older operation acting on a newly exposed site is included, while an
unrelated old catalog entry is not an implicit dependency. Every retained
member carries the origin event of each source declaration. Later
declarations cannot retroactively enlarge \(I^{\mathcal M}_{B_H}(A)\).

The complete canonical encoding of this finite view, including every
inapplicability certificate, is bound into a census certificate. Let
\(R^{\mathcal M}_{B_H}(A)\) be the finite quotient registry containing exactly the
equality/equivalence entries born no later than \(S^+\) whose typed endpoints
lie in the eligible view. In the remaining sections, \(I(A)\) and \(R(A)\)
abbreviate \(I^{\mathcal M}_{B_H}(A)\) and
\(R^{\mathcal M}_{B_H}(A)\) for the fixed boundary and manifest.

### 3.5 Installation manifest and present adoption block

An installable GSC version is a digest-bound finite package:

```text
GscInstallationManifest {
    public_site_code_grammar,
    constructor_code_grammar,
    operation_shape_grammar,
    footprint_and_action_code_grammar,
    comparison_and_mate_problem_grammar,
    relevance_and_compatibility_rules,
    public_port_projection_rules,
    four_interface_compiler_equations,
    canonical_enumeration_order,
    family_and_support_encoding,
    q0_rewrite_manifest,
    q2_q3_atomic_edge_manifest,
    theorem_registry_policy,
    derivability_contract_id
}
```

Every grammar constructor, typing rule, matcher equation, compiler equation,
canonical ordering rule, and source digest is part of the package identity.
Two implementations with different manifests are different calculi even if
both satisfy the architecture below.

This proposal does not choose those finite manifests. Doing so would be the
new normative decision that the prior adopted sources leave open. Therefore
`law-v2-gsc-v1-architecture` is a proposal schema, not an adoption-ready
calculus version. Sections 4--14 state the contracts that a future manifest
must instantiate and prove.

For any use of those sections, first fix one installation manifest
\(\mathcal M\). Every unindexed descriptor, eligible view, quotient registry,
source relation, compiler, canonicalization, and history-derivation judgment
is by definition an abbreviation for its \(\mathcal M\)-indexed form. The
operator definition retains the index explicitly.

## 4. Contexts, ports, premises, and families

### 4.1 Dependent contexts

A context is a finite ordered telescope

\[
  \Gamma=(p_1:M_1,\ldots,p_k:M_k(p_1,\ldots,p_{k-1})).
\]

Every motive is formable when declared. Substitution is sequential and
left-to-right. A motive may be an opaque dereference of a sealed declaration
type. Contexts are deterministic compiler outputs; GSC does not enumerate
arbitrary context syntax.

### 4.2 Required output clauses

A family output is one of two distinct clause kinds:

\[
\begin{array}{ll}
\mathsf{TermPort}(p,\Gamma,M,\rho)
  & \text{requires a term of motive \(M\);}\\[0.3ex]
\mathsf{EquationClausePort}(p,\Gamma,\ell,r,T,o,\rho)
  & \text{requires an oriented exact-extension equation clause.}
\end{array}
\]

Here \(p\) is a canonical port ID and \(\rho\) is a typed structural role. A
type-formation request is a `TermPort` whose motive is a universe. A
path-valued computation is
`TermPort(Path T left right)`, never an `EquationClausePort`.

Term ports form a sequential dependent telescope. Later clauses may depend on
earlier term ports and on earlier equation clauses according to the adopted
sequential exact-extension semantics. A term port is filled by a term.

An `EquationClausePort` is not a claim that
\(\ell\equiv r:T\) already holds in \(B_H\). Its discharge is a
`VerifiedEquationExtensionClause` proving:

1. formation of both sides and \(T\) in the boundary extended by all earlier
   output clauses;
2. the declared orientation \(o\);
3. acceptance by the exact versioned computation-equation grammar;
4. normalization compatibility, substitution stability, and conservative
   preservation of the old boundary; and
5. admissible use by later sequential clauses.

If the equation already holds in the library, the same capability may be
constructed by replay without extending the boundary. The equation
certificate is evidence, not a term metavariable. Individual clause
capabilities do not establish that a union of sibling family clauses is
jointly admissible; that is a separate discharge obligation in Section 12.1.

Output roles belong to the finite grammar:

```text
GlobalOperationPort(operation_shape, footprint_code, action_code)
UsePort(owner_former_id, operation_shape, footprint_code, action_code)
ComputationPort(former_id, intro_port_id, computation_mode)
ActionPort(operation_shape, site_id, left_route, right_route)
ComparisonPort(problem_id, orientation)
MatePort(problem_id, orientation)
PlainTermPort
```

The owed output is not inserted into the parameter context as an assumption.

### 4.3 Premise references

Every generated dependency is explicit:

```text
PremiseRef {
    source: PublicPort(public_port_id)
          | GeneratedPort(family_id, output_port_id),
    expected_role,
    context_map
}
```

For a source family over \(\Gamma_s\) and a target family over \(\Gamma_t\),
`context_map` is a checked substitution
\(\chi:\Gamma_t\to\Gamma_s\). It reindexes the same source output; it does not
create a fresh filler. `GeneratedPort` may target only a port whose clause
kind and output role match `expected_role`.

The enumerated compiler input is only a finite source key:

```text
SourceKey {
    PublicPort(public_port_id)
  | GeneratedPort(family_id, output_port_id)
}
```

It contains no caller-selected context map. The deterministic rule compiler
first constructs \(\Gamma_t\), then its finite canonical alignment routine
returns the unique checked map or `CertifiedInapplicable`/`OutsideFragment`.
The successful map is stored in the resulting `PremiseRef`. GSC never
enumerates an arbitrary substitution grammar.

### 4.4 Canonical family shape

Every rule constructs one finite family object:

```text
CanonicalDemandFamily {
    parameter_context,
    premise_refs,
    output_clause_telescope,
    verification_judgments,
    principal_sources,
    birth_support,
    fixed_type_support,
    parameter_support_projections
}
```

Port IDs are derived from the installation digest, rule constructor,
canonical pre-quotient source IDs, and port position. The source IDs come
from the separately normalized public-site view; they do not depend on the
family quotient. Output clauses and verification judgments are deterministic
compiler results, not independently enumerable syntax.

A rank-\(2\) derivation reuses the exact
`(family_id, output_port_id)` source key of each rank-\(1\) premise. Each
occurrence carries its own checked `context_map`, which reindexes that one
source section; the map is not part of the filler key. A derivation may not
copy a premise into fresh, independently fillable slots.

## 5. Deterministic interface compilers

Every compiler invocation returns exactly one of:

```text
Applicable(CanonicalDemandFamily)
CertifiedInapplicable(checked_reason)
OutsideFragment(reason)
```

`CertifiedInapplicable` is the ordinary negative result for a well-supported
tuple. `OutsideFragment` prevents a complete census. It is never interpreted
as inapplicability. The following four total result-bearing functions are part
of a valid GSC installation:

```text
compile_use
  : ClosedFormerFrame
 -> CompileResult

compile_compute
  : (ClosedFormerFrame, SourceKey<UsePort>, IntroPort)
 -> CompileResult

compile_action
  : (SourceKey<GlobalOperationPort | UsePort>, CompatiblePublicSite)
 -> CompileResult

compile_compare
  : (SourceKey<ActionPort>, SourceKey<ActionPort>,
     ComparisonProblemFrame | MateProblemFrame)
 -> CompileResult
```

They receive only checked eligible-view data and explicit lower-rank source
derivations. They are not deserializable extension points.

### 5.1 `compile_use`

`compile_use(D)` constructs the uniform use interface determined by the
checked constructor code \(D\):

- a motive over the owner;
- one method telescope for each introduction port;
- recursive hypotheses only at recursive occurrences recorded by \(D\);
- one endpoint method and restriction equation for each boundary port; and
- a result uniformly quantified over an owner instance.

For owner family \(A_D(\vec p,\vec i)\), the generated core has the standard
shape:

\[
\begin{aligned}
P &:\prod_{\vec i}\,A_D(\vec p,\vec i)\to\mathcal U,\\
m_c &:\prod_{\vec\xi}\,
      \left(\prod_{r\in\operatorname{RecPos}(c)}
        P(\operatorname{index}_r,\operatorname{arg}_r)\right)
      \to P(\operatorname{index}_c,c(\vec\xi)),\\
u &:\prod_{\vec i}\prod_{z:A_D(\vec p,\vec i)}P(\vec i,z).
\end{aligned}
\]

There is one \(m_c\) for every `IntroPort`; the endpoint ledger extends these
method types for every `CellPort`. The output term port has a `UsePort` role
carrying the operation shape, footprint, and action code needed downstream.
The compiler is structural recursion on the finite constructor-code grammar.
If the code uses a constructor form for which that recursion is not
registered, the result is `OutsideFragment`.

### 5.2 `compile_compute`

For an `IntroPort` \(c\) of \(D\), `compile_compute(D,u,c)` constructs the
computation clause obtained by applying the referenced use port \(u\) to
\(c\). Its right side is the corresponding method applied to exactly the
constructor arguments and recursive hypotheses selected by the recorded
recursive-position mask. Judgmental mode emits an `EquationClausePort`; path
mode emits a `TermPort` with the required path motive. The compiler may not
choose the mode from a later checking outcome.

### 5.3 `compile_action`

An operation source is a public operation port or an explicit lower-rank
generated use-port reference. A compatible public site is accepted by the
source's checked `FootprintCode`. `compile_action` constructs the uniform
action clause determined by its `ActionCode`. For a cell, this is the
dependent path between the operation applied to the cell's two endpoint
routes, with every side condition copied from the checked boundary. The
result exposes an `ActionPort` role containing the typed routes required by a
later comparison.

### 5.4 `compile_compare`

`compile_compare` receives two explicit public or lower-rank action-port
references and a checked `ComparisonProblemFrame` or `MateProblemFrame`. It
constructs the designated missing output prescribed by that frame. Displayed
routes are deterministic frame/compiler outputs from rank-bounded premises;
there is no independent route-length cap or arbitrary parallel-route search.
Reassociation and transparent identity edges are normalized before
compilation.

Every successful compiler result carries proofs of:

- parameter-context, premise-reference, output-clause, and verification
  formation;
- determinism;
- invariance under sealed renaming;
- commutation with typed substitution;
- commutation with weakening; and
- preservation of source support.

Until those proofs replay, the corresponding constructor is unavailable.

## 6. Ranked history-demand rules

For a required output role \(\rho\), a typed source
\(\operatorname{Source}^{q,\mathcal M}_{B_H,A}(\rho)\) is exactly:

1. a compatible public port in \(I(A)\), with \(q=0\); or
2. a premise derivation
   \(B_H;A\vdash_{\mathrm{hist},\mathcal M}^{q}
   d\triangleright F\), an output port
   \(p\in\operatorname{outputs}(F)\) of role \(\rho\), represented only by its
   finite source key.

Thus a generated source includes its derivation, family identity, output-port
ID, and role. The receiving compiler constructs and records the reindexing;
it is never described as a member of \(B_H\).

For a generated derivation,

\[
  \operatorname{rank}(d)
  =
  1+\max\{\operatorname{rank}(p)\mid
           p\text{ is a generated family premise of }d\},
\]

where the maximum of an empty set is \(0\). Only derivations of rank at most
\(2\) enter the carrier. Consequently, no rank-\(2\) family may be used as a
generative premise.

This history-derivation rank is a new GSC contract. It is not inherited from
the old theorem about candidate-local structural-trace arity. Agreement
between those two measures is a separate falsifiable claim.

Write \(\operatorname{birth}(F_d)\) for the origin events of the concrete
principal public-site tuple used by a derivation, together with inherited
birth support of generated principal ports. Type support is defined
separately in Section 7.

### 6.1 Formation/use completion

\[
\frac{
  D\in I(A):\mathsf{ClosedFormerFrame}
  \qquad
  \operatorname{compile\_use}(D)=\mathsf{Applicable}(F)
  \qquad
  \operatorname{jur}_A(F)
}{
  B_H;A\vdash_{\mathrm{hist},\mathcal M}^{1}
  \mathsf{FormationUse}(D)\triangleright F
}
\quad(\mathsf{G\mbox{-}Use})
\]

This derivation has rank \(1\). Its principal tuple is the closed former and
the introduction/cell ports consumed by `compile_use`.

### 6.2 Introduction/computation completion

\[
\frac{
  D\in I(A):\mathsf{ClosedFormerFrame}
  \quad
  u\in\operatorname{Source}^{q,\mathcal M}_{B_H,A}(\mathsf{UsePort}(D))
  \quad q\leq1
  \quad
  c\in I(A):\mathsf{IntroPort}(D)
  \quad
  \operatorname{compile\_compute}(D,u,c)=\mathsf{Applicable}(F)
  \quad
  \operatorname{jur}_A(F)
  \quad 1+q\leq2
}{
  B_H;A\vdash_{\mathrm{hist},\mathcal M}^{1+q}
  \mathsf{IntroductionComputation}(D,u,c)\triangleright F
}
\quad(\mathsf{G\mbox{-}Compute})
\]

If \(u\) is generated, the compiler embeds its exact `PremiseRef`; a
different filler cannot be chosen for the computation family.

### 6.3 Path/cell action completion

\[
\frac{
  o\in\operatorname{Source}^{q,\mathcal M}_{B_H,A}
    (\mathsf{GlobalOperationPort}\mid\mathsf{UsePort})
  \quad q\leq1
  \quad
  s\in I(A)
  \quad
  \operatorname{decide\_compatible}(o,s)=\mathsf{Applicable}
  \quad
  \operatorname{compile\_action}(o,s)=\mathsf{Applicable}(F)
  \quad
  \operatorname{jur}_A(F)
  \quad 1+q\leq2
}{
  B_H;A\vdash_{\mathrm{hist},\mathcal M}^{1+q}
  \mathsf{SupportAction}(o,s)\triangleright F
}
\quad(\mathsf{G\mbox{-}Action})
\]

An existing operation has source rank \(0\), so its action has rank \(1\).
A generated use-family output has source rank \(1\), so its action has rank
\(2\). Any other source is rejected. When \(s\) is a `CellPort`, this is the
path/cell-action case.

### 6.4 Parallel-route comparison completion

\[
\frac{
  a_i\in\operatorname{Source}^{q_i,\mathcal M}_{B_H,A}(\mathsf{ActionPort})
  \quad q_i\leq1
  \quad
  R\in I(A):
  (\mathsf{ComparisonProblemFrame}\mid\mathsf{MateProblemFrame})
  \quad
  \operatorname{compile\_compare}(a_0,a_1,R)=\mathsf{Applicable}(F)
  \quad
  \operatorname{jur}_A(F)
  \quad 1+\max(q_0,q_1)\leq2
}{
  B_H;A\vdash_{\mathrm{hist},\mathcal M}^{1+\max(q_0,q_1)}
  \mathsf{MateComparison}(a_0,a_1,R)\triangleright F
}
\quad(\mathsf{G\mbox{-}Compare})
\]

Its principal tuple contains both action ports and the checked comparison or
mate-problem frame.

### 6.5 Future-hole formation

Each term output above is stored canonically as:

```text
FutureHole {
    family_derivation,
    family_id,
    output_port_id,
    declared_context,
    required_motive,
    output_role
}
```

A computation-equation output is stored analogously as a
`FutureEquation`. These are one-to-one serializations of output clauses of a
completed rule derivation, not freely repeatable inference rules. They
introduce no additional closure level and cannot generate arbitrary motives.

### 6.6 No negative premises

No generation rule asks whether its conclusion is missing, derivable, live,
valuable, or likely to be discharged. `CertifiedInapplicable` concerns only
the syntax-directed applicability of a frozen rule to an eligible source
tuple. Derivability is decided only after the complete carrier is generated.
This is required for extraction completeness and monotonicity.

## 7. Seeds, support jurisdiction, and closure order

### 7.1 Seed relation

The sole rank-\(0\) sources are verified descriptors and already public
typed ports in \(I(A)\). A rule grounding is retained only when every declared
public principal occurs as a rigid, non-erasable declaration reference in the
normalized family it synthesizes. For a generated principal, the exact
premise family and port key must remain in `premise_refs`. A source that
normalizes away cannot
activate a family merely because it appeared in rule provenance.

No arbitrary judgment, motive, output slot, or caller-supplied family is a
seed.

### 7.2 Jurisdiction

Each family \(F\) has two distinct support objects.

`birth_support(F)` is the union of the origin events of concrete principal
public sites and the birth supports of generated principal ports used by its
rule. It is fixed before specialization.

`type_support(F)` is symbolic:

\[
  \operatorname{TypeSupp}_F(\theta)
  =
  \operatorname{Fixed}(F)
  \cup
  \bigcup_{i\in\operatorname{Proj}(F)}
    \operatorname{Supp}(\theta_i).
\]

The fixed declaration set is typed recursively:

\[
  \operatorname{Fixed}(F)
  =
  \operatorname{CompilerConstants}(F)
  \cup
  \operatorname{PublicPrincipalDecls}(F)
  \cup
  \bigcup_{p\in\operatorname{premise\_refs}(F)}
    \operatorname{FixedSource}(p).
\]

`FixedSource` is the public port's verified declaration dependency closure or
the referenced generated family's `Fixed` set, respectively. `Fixed(F)` and
the exact parameter
projections `Proj(F)` are computed from normalized occurrence and transitive
dependency analysis. Every fixed declaration reference must resolve in
\(B_H\); every generated key must occur in `premise_refs`. Assignments may
enlarge type support but cannot alter birth support.

A derivation has anchor jurisdiction exactly when

\[
  \operatorname{birth}(F)
  \cap
  \{S^+,S^-\}
  \neq\varnothing.
\]

Thus an older public operation acting on a newly exposed site is a fresh
grounding whose principal tuple includes that site. Conversely, substituting
new content into an old generic parameter cannot reactivate an expired
family. Quotient witnesses must transport both birth support and symbolic
type support.

### 7.3 Closure order

The carrier is constructed in this fixed order:

1. descriptor extraction and seed validation;
2. exhaustive compiler disposition for every source tuple whose calculated
   rank is \(1\);
3. exhaustive compiler disposition for every source tuple whose calculated
   rank is \(2\);
4. future-hole packaging;
5. normalization; and
6. finite quotient-edge enumeration and graph closure.

No fixed point over arbitrary judgments is taken. "Least" means exactly the
inductive closure under the four displayed constructors at their declared
ranks.

## 8. Families are born as families

For a derivation \(d\), define:

```text
FamilyIdentity {
    installation_digest,
    rule_constructor,
    normalized_parameter_context,
    normalized_premise_refs,
    normalized_output_clauses,
    normalized_verification_judgments,
    canonical_prequotient_source_ids,
    birth_support,
    fixed_type_support,
    parameter_support_projections
}
```

The identity is the digest of this complete canonical encoding and is computed
before any closed assignment. The source IDs use the public-site
normalization fixed before GSC family generation, not a family-equivalence
orbit.

A specialization is

```text
InstanceOf {
    family_id,
    sequential_substitution,
    normalized_specialized_output_clauses,
    normalized_specialized_verification_judgments
}
```

and never becomes a second family. Two concrete goals that merely resemble
one another are not inferred to share a family; shared identity must descend
from one derivation or from a checked quotient witness.

## 9. Normalization and finite quotient

Let \(\operatorname{NFQ}^{\mathcal M}_{B_H,A}\) be the following layered
normalization and closed-vertex quotient.

### Q0. Judgmental normalization

Normalize de Bruijn binders, sequential substitution,
\(\beta/\iota/\zeta\)-redexes, the explicitly registered \(\eta\)-rules,
projections, transparent aliases, and canonical dependent-telescope
flattening. Every rewrite is typed and source-digest bound.

### Q1. Natural-family identity

All `InstanceOf` values retain their originating family identity. Renaming of
bound parameters and specialization do not split a family.

### Q2. Presentation witnesses

The finite presentation grammar contains exactly:

```text
ReassociateDependentSum
SplitSingleConstructorRecord
BundleSingleConstructorRecord
CurryDependentFunction
UncurryDependentFunction
ExchangeIndependentDeclarations
TransportAlongCertifiedPublicEquality
InsertTransparentAlias
DeleteTransparentAlias
DeleteDuplicateCertifiedTransparentField
```

Each primitive witness carries canonical typed maps in both directions, a
bijection of output ports preserving clause kind and role, commuting maps for
premise references, and certificates that parameter context, verification
judgments, birth support, symbolic type support, and family substitution
action are preserved.

`DeleteDuplicateCertifiedTransparentField` requires a local pre-quotient
synthesis/transparency certificate. It never consults the later
`Derived`/`Underived` disposition.

### Q3. Explicit equivalence witnesses

The atomic equivalence-edge grammar contains exactly:

```text
PiCongruence
SigmaCongruence
FiniteSumCongruence
PathCongruence
TelescopeExchange
SealedRenaming
RegisteredCubicalEquivalence { theorem_digest }
```

`Refl` is graph identity, `Sym` reverses an atomic edge, and `Trans`
concatenates graph paths. They do not construct additional recursive witness
ASTs.

The vertex set is fixed before Q2/Q3:

\[
  N^{\mathcal M}_{B_H,A}
  =
  \{\operatorname{Q0}(F_d)\mid
    B_H;A\vdash_{\mathrm{hist},\mathcal M}^r
      d\triangleright F_d,\ r\leq2\}.
\]

Presentation operations, including alias insertion, bundling, and currying,
only test relations between vertices already in \(N^{\mathcal M}_{B_H,A}\);
they never
synthesize a new vertex. Registered cubical edges range over the finite
sealed theorem registry \(R^{\mathcal M}_{B_H}(A)\). GSC performs no open-ended theorem
search.

For every atomic constructor, every ordered tuple of existing vertices, and
every required registry entry, the checker returns exactly one of:

```text
Inapplicable(checked_reason)
Edge(checked_equivalence)
Unknown(reason)
```

`Inapplicable` is the normal certified negative disposition. `Unknown`
aborts quotient completeness. Let \(E^{\mathcal M}_{B_H,A}\) be the set of all returned
`Edge` values. The quotient is the connected-component partition of the
finite graph
\((N^{\mathcal M}_{B_H,A},E^{\mathcal M}_{B_H,A})\) after every tuple has
received a
non-unknown disposition. A completeness certificate is always relative to
this exact closed vertex set and atomic-edge grammar.

## 10. Definition of \(C^{\mathcal M}_{B_H}(A)\) and the live profile

Let

\[
  \operatorname{Der}^{\mathcal M}_{\leq2}(B_H,A)
  =
  \left\{
    (d,F)\mid
    B_H;A\vdash_{\mathrm{hist},\mathcal M}^r
      d\triangleright F,\ r\leq2
  \right\}.
\]

The history-demand operator is defined by:

\[
  C^{\mathcal M}_{B_H}(A)
  :=
  \operatorname{NFQ}^{\mathcal M}_{B_H,A}
  \bigl(\operatorname{Der}^{\mathcal M}_{\leq2}(B_H,A)\bigr)
  =
  \pi_0(N^{\mathcal M}_{B_H,A},E^{\mathcal M}_{B_H,A}).
\]

This equation is the definitional contract for any manifest-bound installation
of this architecture. Before the Section 3.5 manifests are adopted, it does
not denote one unique executable carrier. After binding, an implementation is
sound and complete only when its carrier and quotient replay exactly this
definition.

Where a Law-V2 history \(H\) has already fixed its verified boundary \(B_H\),
and its certificate has fixed \(\mathcal M\), the shorter notation \(C(A)\)
abbreviates \(C^{\mathcal M}_{B_H}(A)\); it never suppresses an unfixed
boundary or installation.

For a class \(c\in C^{\mathcal M}_{B_H}(A)\),
`Derived` relative to derivability contract \(\mathcal D\) means there is a
coherent
library-only discharge of the full generated-premise downward closure of one
representative, natural in its parameters, together with checked transport to
every representative. Write these dispositions as
\(\operatorname{Derived}^{\mathcal D}(B_H,c)\) and
\(\operatorname{Underived}^{\mathcal D}(B_H,c)\). The latter means complete
finite replay of the adopted discharge basis proves that no such library-only
discharge exists. Failure to construct either certificate is `Unknown`.

The live obligation profile is:

\[
  O^{\mathcal M,\mathcal D}(H)
  =
  \{c\in C^{\mathcal M}_{B_H}(W_H)\mid
    \operatorname{Underived}^{\mathcal D}(B_H,c)\}.
\]

If any class has an undecided derivability or quotient disposition, no
complete live profile is issued. A candidate discharges this profile with one
coherent environment for the downward closure of all live classes; it may not
choose unrelated fillers for shared premise-port keys. The environment must
commute with every Q2/Q3 output-port bijection and must import the
library-only discharge of every downward premise already classified
\(\operatorname{Derived}^{\mathcal D}\).

Adoption of the history-generation and quotient manifests would make only
\(C^{\mathcal M}_{B_H}(A)\) normative. The displayed
\(O^{\mathcal M,\mathcal D}(H)\) becomes executable and authoritative only
when `derivability_contract_id` resolves to a separately versioned finite
discharge/saturation manifest defining `Derived` and `Underived`. Until then
every live-profile request is `Unknown`.

## 11. Structural finiteness theorem

Assume:

1. the verified cumulative boundary \(B_H\), its sealed events, and all
   reachable opaque dependency records are finite;
2. descriptor extraction and eligible-view closure terminate with no
   `OutsideFragment`;
3. the four compilers give a non-unknown disposition for every eligible
   source tuple;
4. every constructor has finite declared arity; and
5. Q0--Q3 checking terminates on generated families and the finite anchored
   proof registry.

Then \(C^{\mathcal M}_{B_H}(A)\) is finite.

### Proof

\(I(A)\) is finite by assumptions 1 and 2. Rank-\(1\) derivations are formed
by finitely many choices of a rule and a finite tuple of members of
\(I(A)\).
Rank-\(2\) derivations are formed by finitely many choices of a rule, finite
descriptor tuples, and rank-\(1\) premises. No rule consumes a rank-\(2\)
premise.

If \(D_q\) is the set of derivations of rank \(q\), a coarse structural bound
that includes mixed descriptor and generated-family premises is

\[
  |D_{r+1}|
  \leq
  \sum_{R\in\mathcal R_{\mathrm{GSC}}}
  \left(
    |I(A)|+\sum_{q\leq r}|D_q|
  \right)^{a_R},
  \qquad r<2,
\]

where every \(a_R\) is a fixed finite upper bound on that constructor's total
source arity. Contexts and
conclusions are deterministic compiler outputs, so no independent term or
context enumeration multiplies this set. Future-hole packaging is
one-to-one. Therefore
\(\operatorname{Der}^{\mathcal M}_{\leq2}(B_H,A)\) is finite.
Normalization maps a finite set to a finite set. Q2/Q3 inspect finitely many
ordered vertex tuples and finite registry entries without adding vertices,
so their graph and its component quotient are finite. \(\square\)

This theorem does not say that arbitrary well-typed GF2 terms are finite.
It avoids that false requirement by making canonical rule derivations the
carrier.

## 12. Required induction theorems

The following are adoption gates, not claims already established by this
Markdown proposal.

### 12.1 Total formation and specialization

For every derivation
\(B_H;A\vdash_{\mathrm{hist},\mathcal M}^r
d\triangleright F\), compiler induction
proves:

\[
\begin{aligned}
B_H&\vdash\Gamma_F\ \mathsf{ctx},\\
B_H;\Gamma_F
  &\vdash\operatorname{premises}(F)\ \mathsf{ok},\\
B_H;\Gamma_F,\operatorname{premises}(F)
  &\vdash\operatorname{outputs}(F)\ \mathsf{clause\mbox{-}tel},\\
B_H;\Gamma_F,\operatorname{premises}(F),\operatorname{outputs}(F)
  &\vdash\operatorname{verification}(F)\ \mathsf{wf}.
\end{aligned}
\]

These judgments remain valid under every kernel-admissible sequential
parameter assignment and every checked premise `context_map`. This proves
formation of every specialized term motive, definitional equation, and
verification judgment. It does not construct a demanded term or equation
certificate.

For a finite downward-closed family DAG \(K\),
\(\operatorname{Discharge}^{\mathcal M,\mathcal D}_{B_H}(K)\) is a coherent
natural assignment under the derived/live partition certified by
\(\mathcal D\) that:

1. maps every `(family_id, term_port_id)` to one term section;
2. maps every `(family_id, equation_port_id)` to one proposed oriented
   equation clause;
3. constructs the canonical global clause order extending every family
   telescope order and every premise dependency order, breaking independent
   ties by canonical port key;
4. carries one `VerifiedEquationExtensionSet` proving that the complete
   ordered union of equation clauses is jointly typed, admissible,
   conservative over \(B_H\), normalization-compatible, confluent under the
   adopted rewrite discipline, and stable under substitution;
5. reuses each source section through every premise context map;
6. commutes with every quotient-edge output-port bijection;
7. imports the certified library discharge of every derived downward
   premise; and
8. satisfies every verification judgment.

Discharge specialization is a separate structural theorem. It transports one
such environment along every parameter assignment and never confuses
well-formedness with existence.

### 12.2 Equivariance

Every checked isomorphism of finite anchored eligible views and quotient
registries

\[
  (I^{\mathcal M}_{B_H}(A),R^{\mathcal M}_{B_H}(A))
  \simeq
  (I^{\mathcal M}_{B_{H'}}(A'),R^{\mathcal M}_{B_{H'}}(A'))
\]

induces a bijection

\[
  e_*:C^{\mathcal M}_{B_H}(A)
       \simeq C^{\mathcal M}_{B_{H'}}(A')
\]

that commutes with rule constructors, family identity, specialization,
origin and dependency data, footprint matching, compiler bindings, support,
normalization, and quotient witnesses.

### 12.3 Weakening

For a conservative verified boundary inclusion
\(i:B_H\hookrightarrow B_{H'}\) preserving old event identities and
normalization, structural recursion defines:

\[
  B_H;A\vdash_{\mathrm{hist},\mathcal M}^r d\triangleright F
  \Longrightarrow
  B_{H'};iA\vdash_{\mathrm{hist},\mathcal M}^r
  i(d)\triangleright i(F).
\]

Weakening preserves family identity by transport, specialization, support,
port keys, context maps, normal forms, quotient edges, and discharge evidence.
The cutoff in \(I^{\mathcal M}_{B_{H'}}(iA)\) excludes declarations born after the newest
event of \(A\), preventing retroactive groundings. Here \(iA\) is an anchor,
not necessarily the active pair of \(H'\).

### 12.4 Window locality

If two histories have isomorphic finite eligible views and quotient
registries in the sense of Section 12.2, their anchored carriers and
quotients are canonically equivalent. This is the precise locality boundary.
The construction may inspect pre-anchor standing descriptors only through
the exhaustive matcher used to construct that view. It may not inspect
post-anchor declarations, later history, human metadata, or candidate data.

### 12.5 Expiration

The ledger invariant is open-family discharge:

\[
  \forall e\in\operatorname{PastLedger}(H),\quad
  \operatorname{Discharge}^{\mathcal M,\mathcal D}_{B_H}
    (\downarrow F_e).
\]

Every past-ledger entry binds the installation digest \(\mathcal M\), the
derivability-contract digest \(\mathcal D\), and the transported
derived/live partition used by its discharge.

After sealing \(x\):

1. verify one coherent discharge environment for the current live DAG;
2. verify the conservative inclusion \(B_H\hookrightarrow B_{H+x}\);
3. weaken and replay every earlier discharge in \(B_{H+x}\);
4. bind all transported family, port, and proof digests; and
5. remove an entry from the active ledger only when its discharge replays and
   its birth support is disjoint from the shifted active anchor \(W_{H+x}\).

Falling outside the active window is never, by itself, evidence that an
obligation disappeared.

## 13. Certificate obligations

An authoritative census must bind:

- the exact installation manifest \(\mathcal M\), derivability contract
  \(\mathcal D\), and all source digests;
- normalized event order, declaration-origin map, anchor cutoff, and
  dependency-closure digests;
- the complete eligible view, quotient registry, extraction proof, and every
  compatibility disposition;
- every rank-\(1\) and rank-\(2\) derivation;
- proof that all constructor tuples were exhausted;
- compiler outputs, inapplicability dispositions, and formation certificates;
- family identities, output-port roles, premise context maps, and support;
- Q0 normal forms;
- every Q2/Q3 tuple disposition, primitive edge, and the resulting partition;
- a derivability or underivability certificate for every quotient class;
- coherent downward-closure discharge environments where derivability is
  claimed;
- total-specialization, equivariance, weakening, locality, and expiration
  theorem digests; and
- any `OutsideFragment` or resource exhaustion result.

An omitted constructor tuple, unchecked quotient edge, unresolved class, or
missing theorem digest makes the result `Unknown`.

## 14. Blind-prefix falsification run

After adoption and mechanization, the registered anonymous founding prefix,
selected only by its verified source digest, is run through GSC with:

- all human decoder data removed;
- all expected demand families and candidate fixtures unavailable;
- opaque identifiers freshly permuted; and
- the GSC version frozen before execution.

External fixture aliases are not available to descriptor extraction, rule
generation, normalization, quotienting, or derivability.

The run is a falsifier, not a target:

1. the generic rules may issue one completed use interface;
2. they may issue a weaker interface with additional lawful dischargers; or
3. they may issue several unrelated live classes.

Every outcome is reported without changing the descriptors, compilers, rules,
support predicate, or quotient. If the historical expectation is not
recovered, the result identifies a missing bridge principle; it is not a
license to add an answer-shaped constructor.

Additional required falsifiers are identifier permutation, rule ablation,
transparent-alias insertion, one injected underived goal, and constrained
resource replay.

## 15. Nonclaims and adoption boundary

This proposal does not establish:

- that its rule inventory is the uniquely correct account of prospective
  demand;
- that the descriptor or interface compilers have been implemented;
- that GF2 satisfies every compiler theorem;
- that GF2 translates adequately to the old sealed-extension calculus;
- that higher candidate-local trace is derived in Atomic;
- that the current relative census identifies
  \(C^{\mathcal M}_{B_H}(A)\) or that its opaque reachability is GSC
  `Derived`;
- that any particular prefix has a particular live profile;
- response enumeration, candidate factorization, full constitutive
  certification, free-sealing initiality, autonomous continuation, or
  autonomous termination; or
- a least or unique founding prefix.

Adoption requires an explicit versioned decision on:

1. the verified event/origin model, eligible-view construction, descriptor
   grammar, and compiler;
2. the birth-support jurisdiction and symbolic type-support clauses;
3. operation/footprint/action codes, comparison/mate problem frames, and the
   four deterministic interface compilers;
4. the canonical family/output-port model, ranked constructors, and premise
   reference discipline;
5. Q0--Q3;
6. the definition of `Derived` and complete finite `Underived`;
7. compatibility or supersession relative to earlier carrier
   adjudications; and
8. replayable proofs of the five induction theorems.

### 15.1 Earliest representation blockers

The current verified registered bootstrap proves its exact anonymous
one-declaration chain and final normalized signature. Existing kernel
capabilities also verify signatures, contexts, derivations, definitional
equivalences, particular intrinsic specializations, and exact requested
extensions. They do not yet expose:

1. a verified per-event public view with a declaration-to-origin-event map;
2. a public descriptor-bearing group capability proving constructor-package
   completeness, strict positivity, roles, recursion masks, and computation
   mode;
3. a `VerifiedHistory`/`VerifiedAnchor` capability carrying event order,
   immediate-predecessor evidence, prefix cutoffs, persistent old-anchor
   replay, and active-tip selection rather than only a caller-supplied pair
   of global declaration IDs;
4. a first-class unsolved family model separating parameters, premise refs,
   term/equation output ports, and verification judgments; or
5. resolvable typed GF2 artifacts for constructor, path/cell, and structural
   codes; and
6. a verified origin-cutoff equality/equivalence registry
   \(R^{\mathcal M}_{B_H}(A)\), not merely individual tokens or inventory
   digests.

The current declaration syntax distinguishes axioms from definitions, but
that is insufficient to classify a constructor intrinsically under a generic
adopted rule. The GF2 adapter currently reports its non-native artifact path
as unavailable. Implementation therefore needs either conservative
kernel-facing representations for these capabilities or a reviewed
GF2-to-certificate translator/checker bridge.

### 15.2 Remaining proof and execution blockers

After those earliest representations exist, authority still requires:

- the syntax-directed descriptor extractor and deterministic compiler
  implementations;
- GSC family IDs and support objects that bind the full encoding rather than
  the current motive/body-only identities;
- Q0 termination and complete Q2/Q3 tuple disposition;
- structural total-specialization and quotient-transport theorems;
- a typed GSC derivability/discharge representation and adapter capable of
  multi-output families, shared premise-port DAGs, coherent natural
  environments, and quotient transport;
- `VerifiedEquationExtensionClause` and
  `VerifiedEquationExtensionSet` checkers for oriented equations, canonical
  joint ordering, conservativity, normalization/confluence, and substitution
  stability;
- a finite adopted discharge basis and complete `Underived` procedure; and
- weakening, locality, and expiration certificates.

The existing relative census already replays a finite caller-supplied
domain/rule/seed closure and exact active-minus-reached complement. Its
single-motive families and opaque instance-ID edges are not yet a typed GSC
derivability representation. After the new representation and adapter exist,
the remaining theorem must show that the GSC-generated domain, rules, seeds,
and quotient are complete and that typed reachability is exactly GSC
`Derived`. Unchecked history/census DTOs confer no authority.

Until the versioned adoption and these implementations, the external-prefix
census blocker remains in force and the only lawful engine result is
`Unknown`.
