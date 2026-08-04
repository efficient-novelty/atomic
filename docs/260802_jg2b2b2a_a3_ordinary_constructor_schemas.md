# JG2b2b2a-A3-O Ordinary Constructor Realization Schemas

Date: 2026-08-02

Status: **JG2b2b2a-A3-O BASE SPECIFICATION FROZEN; POST-R2b A3-C1 WIRE
CORRIGENDUM ACTIVE WITH R2c (2026-08-02).** This record instantiates the
seven ordinary constructor-instance families, finite source-slot grammars,
checked realization predicates, principal-root selectors, and exact later
theorem subjects over the frozen A2-O carrier/action substrate. It proves no
subject and mints no executable classifier or indexed-interface authority.
The attempted A4-O RC1 binding of A1, A2-O, and this record failed its release
audit; the `ordinary-dependent-envelope-v1` profile is therefore not yet
definition-closed or selectable. A4-R1 signature/source closure and A4-R2a's
acyclic schema/path/ownership foundation and A4-R2b exact
Level-I/raw/decoded/built and early-Level-II payload/tag schemas are frozen.
Active R2c and then R2d close the late payloads and joined root `0xe3` before
A4-R3, the post-R3 R2e integration join, and A4-R4 fixtures.

A3-C1 closes one omission exposed while making R2c support executable: section
4.2 already required every structural obligation record to emit an
`ObligationPremise` dependency even when it contains no `Global` or `Var`, but
the base expected-judgment sum had no honest tag for a structural record or an
exact construction rule. The separately ordered corrigendum is
`docs/260802_jg2b2b2a_a3_c1_expected_judgment_corrigendum.md`. Its dependency
order is explicitly `A3-O base -> A4-R2a -> A4-R2b -> A3-C1 -> A4-R2c`, so
R2a and R2b do not consume their own downstream type definitions. It adds no constructor, carrier,
theorem, proof, or runtime authority; base tags 0--7 and every other A3 wire
order remain unchanged.

This record depends on:

- the A1 common envelope protocol in
  `docs/260802_jg2b2b2a_adequacy_and_envelope_protocol.md`;
- the A2-O carrier/action substrate and parametric proposal operator in
  `docs/260802_jg2b2b2a_indexed_ontology_profiles.md`;
- the exact JG2a seven-constructor order;
- the JG2b2b0 occurrence grammar; and
- the JG2b2b1 particular substitution orientation and replay protocol.

## 1. Scope and interpretation

The seven nominal JG2a arrows classify *constructor instances*. They do not
assert that the nominal input sorts determine a unique output. Output carrier
values, hidden dependent indices, raw realization data, and proof receipts are
quantified by the constructor family even when their sort names disappear
under nominal erasure. This is necessary, for example, because `Here(Gamma)`
does not determine one interface and `(G,S)` does not determine one activation
section.

The schema key is supplied only by the verifier's internal iteration over this
closed order:

1. `Formation`;
2. `Abstraction`;
3. `Aggregation`;
4. `Transport`;
5. `Comparison`;
6. `DemandCompiler`; and
7. `DischargeTransformer`.

No raw code contains a constructor enum, role enum, expected output, expected
judgment, match bit, historical label, Genesis target, or desired result.

Write:

```text
PI(Gamma)             = PublicInterface[Sigma,Gamma]
Fam(Gamma,A,B)        = InterfaceFamily[Sigma,Gamma,A] with value B
Sub(Delta,Gamma)      = Substitution[Sigma,Delta,Gamma]
Cmp(Gamma,A,B)        = ComparisonWitness[Sigma,Gamma,A,B]
G = (E,O)             = SealedPublicGrammar[Sigma,Gamma]
S = (X,ports,contracts)= DemandScheme[Sigma,Gamma,G]
Spec(S,a)             = the deterministic A2-O specialization LiveDemand
Dch(Gamma,G,S,L)      = Discharge[Sigma,Gamma,G,S,L].
```

Telescope concatenation is written `A.B`. If `A in PI(Gamma)` and
`B in PI(Gamma.A)`, then `A.B in PI(Gamma)` is the oldest-first concatenation
of their already dependent entries. It does not form a `Pi`, form a `Sigma`,
flatten either constructor, synthesize a term, or identify presentations.

## 2. Closed finite raw-code and realization contract

For a fixed verified `(H,o,c)`, every check uses A4-R1's exact opaque
evaluation signature `Sigma(H,o)`, while the separate full post-birth
historical successor is provenance evidence only. A2-O supplies the finite source census
`V_b=U_b disjoint-union Pub_<b(H)` and `N=|V_b|`. This section closes the
previously parametric operator by defining its leaf codecs, bounded compound
codecs, seven raw-code products, and provenance projection.

Frozen A4-R1 specifies the census and raw-universe order without changing these products:
`U_b` is sorted by full canonical `OccurrenceId14V1` bytes,
`Pub_<b(H)` by
`(ExportIdV1,PublicFieldPathV1::DeclarationField)`
canonical bytes, and
`V_b=Current(U_b) ++ OlderPublic(Pub_<b(H))`. The anchor receives no priority.
Duplicate exact identities abort source-census history replay.

### 2.1 Source leaves and partial decoders

The only raw syntax-bearing identity is

```text
SourceRefV1 ::= Current(OccurrenceId14V1)
              | OlderPublic(ExportIdV1,DeclarationField).

SourceLeafV1 =
  (source_ordinal:u32, source:SourceRefV1).
```

`source_ordinal` must select byte-identical full `SourceRefV1` bytes in the
owned `BirthSourceCensusV1`. It is retained in every raw leaf and is never a
replacement for the full source identity. Subsequent `SourceRef` notation in
the mathematical decoder displays means `SourceLeafV1.source`; every actual
raw record retains the enclosing ordinal.

`Current(q)` decodes to the exact normalized subterm at `q`, its exact
JG2b2b0 structural path, and its derived local binder context `Lambda_q`.
`OlderPublic(x)` decodes only to the exact public global term and normalized
public type retained by opaque export `x`; it has no private-body projection.
The global is checked and normalized only in `Sigma(H,o)`, whose construction
has erased every strictly older body before kernel replay.
The leaf projection is deliberately **value-only**: `TypeView` tests
`Global(x)` itself for `TypeFormation`, while the declared normalized public
type is checker metadata for `TermView`.  The decoder never silently uses that
declared type as a second syntax source.  Adding such a source would require a
new finite `Value | DeclaredType` projection tag and a new profile version.

There is no arbitrary context map hidden in source decoding. Define the sole
admissible current-source map by canonical prefix weakening. If

```text
Lambda_q = (z_0:Z_0,...,z_(m-1):Z_(m-1))
Gamma    = Lambda_q.C
|C|      = r,
```

where the oldest prefix has the same ordered JG2b2b0 binder-occurrence
identities as `Lambda_q` and its types pass prefix-wise normalized-context
equality, then

```text
wk_(Lambda_q,C) : Gamma -> Lambda_q
```

is the JG2b2b1 substitution whose oldest-codomain-first image vector is

```text
(Var(m+r-1), Var(m+r-2), ..., Var(r)).
```

The verifier derives every image type sequentially and checks this projection.
The decoder retains

```text
WeakeningViewV1(q,Gamma)
  = (source_binder_keys, target_prefix_binder_keys, shift=r,
     oldest-codomain-first image vector, sequential typing replay).
```

This vector and record are unique; `C` empty gives identity. If the exact
binder-occurrence keys differ, or `Lambda_q` is not the oldest normalized
prefix of `Gamma`, the view is false even when the displayed types happen to
normalize alike. For an older public global, closed global weakening into any
`Gamma` is the unique admissible map and the decoder retains the empty source-
binder map. No strengthening is admitted in this profile version.

The following decoders are therefore partial, deterministic functions:

```text
TypeView(Current(q),Gamma)
  : q's source term reindexed only by wk_(Lambda_q,C) and checked by
    TypeFormation;
TypeView(OlderPublic(x),Gamma)
  : Global(x) under unique closed weakening, checked by TypeFormation;
TermView(Current(q),Gamma,T)
  : q's source term reindexed only by wk_(Lambda_q,C) and checked by HasType
    at internally derived T;
TermView(OlderPublic(x),Gamma,T)
  : Global(x) under unique closed weakening, checked by HasType at T;
CtxCode ::= Empty | LocalPrefix(Current(q),j),
            1 <= j <= BinderDepth(q).
```

`CtxCode` decodes to the empty context or the first `j` entries of the exact
oldest-first local binder context derived from `q`. An ill-formed view is a
checked code rejection. There is no caller context, arbitrary substitution,
unification search, or fallback elaboration.
The empty context has only the `Empty` code; a zero-length local-prefix locator
cannot introduce an otherwise unused auxiliary dependency.

Every telescope extension must also assign a binder identity; its type alone
is insufficient for the exact-prefix test above. For a `SourceLeafV1 s` at
full `RawFieldPathV1 p`, freeze

```text
IntroducedBinderKey(s,p) =
  HistoricalBinder(parent(q),BodyEdge)
    if s.source=Current(q), q is exactly the parameter-type child of a Pi, Sigma,
    or Lambda node, and BodyEdge is respectively PiBody, SigmaBody, or
    LambdaBody at that same exact parent occurrence;
  SyntheticBinder(ordinary-profile-version,c,p)
    otherwise.
```

Synthetic keys are injective in `(profile-version,c,p)` and disjoint from all
JG2b2b0 historical binder keys.  `OlderPublic` therefore always introduces a
synthetic key.  `TelCode` appends this key with each decoded field and retains
it in the decoded context and `FieldLineageV1`.  Consequently a later current
source can see a newly extended target as its exact historical prefix only
when the source sequence really follows the corresponding binder ancestry;
normal-form coincidence cannot manufacture that relation.

`BinderKeyV1::FormalBinder(subject_path:SubjectPathV1,binder_ordinal:u32)` is
the third disjoint binder-key variant. It is used only in checked formal-action
contexts, never emitted by a raw-code decoder or admitted in a candidate.

### 2.2 Bounded compound codecs

Every length field below satisfies `0 <= n <= N`; a nonempty length satisfies
`1 <= n <= N`; and a source ordinal satisfies `0 <= i < N`. Every vector is
oldest-first, has its stated exact length, and contains `SourceLeafV1` values
whose full `source` bytes are pairwise distinct across the complete enclosing
raw code. One used-source bitset keyed by checked `source_ordinal` spans that
whole code. The compound codecs are:

```text
TelCode(Gamma)
  = (n; s_0,...,s_(n-1)),
    decoded sequentially by TypeView in Gamma.T_0...T_(i-1).

TelCode+(Gamma)
  = TelCode(Gamma) with 1 <= n <= N.

SecCode(Gamma,A)
  = (s_0,...,s_(|A|-1)),
    decoded sequentially by TermView at the internally substituted field type.

SubCode(Delta,Gamma)
  = SecCode(Delta,Gamma), checked in the exact JG2b2b1 orientation and replayed
    as one internally generated particular substitution.

FamilyCode(Gamma)
  = (A:TelCode(Gamma), B:TelCode(Gamma.A)).

GrammarCode(Gamma)
  = (E:TelCode(Gamma); p;
     (P_i:TelCode(Gamma.E),
      R_i:TelCode(Gamma.E.P_i))_(i<p)).

SchemeCode(Gamma,G=(E,O))
  = (X:TelCode(Gamma.E); m;
     (k_i,args_i)_(i<m); h; (type_j,left_j,right_j)_(j<h)),
```

where every `k_i` is in `0..|O|-1`; `args_i` is the exact `SecCode` for the
selected operation parameters in the prefix containing prior derived outputs;
and every contract code is parsed in the displayed `type,left,right` order:
first `TypeView(type_j)` derives the checked type in the final prefix, then
`TermView(left_j,...,type_j)` and `TermView(right_j,...,type_j)` check the two
endpoints.  `Build_c` stores the A2 carrier's canonical external tuple order
`(left,right,type)`.  Port output types are derived by substitution and are not
raw leaves.

```text
DischargeCode(Gamma,G,S,a)
  = SecCode(Gamma.E,Spec(S,a).DemandedTelescope),
```

followed by deterministic substitution into every specialized contract and
independent kernel equality replay. `Spec(S,a)`, comparison replay,
normalization/support evidence, primitive certificates, theorem endpoints, and
the principal-root selector are derived records, never raw-code fields.

The codec nesting above is fixed-depth. No codec refers to itself, and no list
can exceed `N`; therefore every codec is a finite product/sum over `V_b`.

### 2.3 Exact seven raw-code products

After each preceding *raw shape* fixes the next finite arity, the seven
products are:

```text
RawCode_Formation
  = (Gamma:CtxCode, A:TelCode+(Gamma)).

RawCode_Abstraction
  = (Xi:CtxCode, cut, B:TelCode+(Xi)),
    where Xi=Gamma.A at cut and A is a nonempty suffix.

RawCode_Aggregation
  = (Gamma:CtxCode, (A,B):FamilyCode(Gamma), C:TelCode+(Gamma)).

RawCode_Transport
  = (Delta:CtxCode, Gamma:CtxCode, theta:SubCode(Delta,Gamma),
     A:TelCode+(Gamma), C:TelCode+(Delta)).

RawCode_Comparison
  = (Gamma:CtxCode, A:TelCode+(Gamma), B:TelCode+(Gamma)).

RawCode_DemandCompiler
  = (Gamma:CtxCode, G:GrammarCode(Gamma), S:SchemeCode(Gamma,G),
     a:SecCode(Gamma.E,S.X)),
    with |S.X|>0 and |S.ports|+|S.contracts|>0.

RawCode_DischargeTransformer
  = (Delta:CtxCode, Gamma:CtxCode, theta:SubCode(Delta,Gamma),
     G:GrammarCode(Gamma), S:SchemeCode(Gamma,G),
     a:SecCode(Gamma.E,S.X), d:DischargeCode(Gamma,G,S,a),
     d_raw:DischargeCode(Delta,G[theta],S[theta],a[theta^E])),
    with |Spec(S,a).DemandedTelescope|>0.
```

The `cut` field ranges over `0 <= cut < |Xi|`, exactly the boundaries that
leave a nonempty suffix. An operation selector satisfies `0 <= k < p`, where
`p` is the earlier raw operation-vector length. Dynamic section, port, and
discharge arities are determined only by earlier raw telescope/result lengths,
never by success of a kernel judgment. All hidden semantic indices in later
displays are deterministic decoder outputs of these products. Thus
`Raw_ord[Slots_c](V_b)` means exactly `RawCode_c`, not an implementation-owned
slot map.

The raw universe is structural: sums use displayed variant order; naturals are
ascending; a source uses its `V_b` order; vectors are length-first and then
lexicographic; products are lexicographic in displayed field order; and
dependent branches are depth-first. `Empty` precedes `LocalPrefix`, whose
choices are current-source order followed by ascending `j`. A repeated-source
prefix is a visited rejected search node but emits no complete code. Semantic
`Decode_c` is called exactly once after a complete structural code exists, so
a typed rejection never prunes an implementation-dependent subtree. The
repaired A4-R3 contract must fix checked preflight/cardinality replay and
contiguous `u32` raw ordinals before this profile can be selected.

### 2.4 Provenance path codec and selector

`OriginPathV1` is a seven-constructor finite enum with exactly one inhabitant
per schema:

```text
FormationOldestOutputType
AbstractionOldestFamilyType
AggregationOldestRawOutputType
TransportOldestRawOutputType
ComparisonOldestLeftType
DemandCompilerOldestActivationImage
DischargeTransformerOldestRawOutputImage.
```

For a successfully decoded `RawCode_c`, `leaf_c(root_c,r)` is the exact
`SourceLeafV1` at the field/image position named by this enum. It is total
because every named vector is required nonempty. Define

```text
origin_r(root_c) = {q}  if leaf_c(root_c,r).source=Current(q),
                   empty if leaf_c(root_c,r).source=OlderPublic(x).

anchor_c(r) = q  iff leaf_c(root_c,r).source=Current(q).
```

Normalization retains this leaf-to-field provenance map, so
`origin_(nf(E))(root_c)` is its replayed image, not a search through normalized
bytes. The seven `root_c` constants are the corresponding enum inhabitants.
A successful build requires the partial `anchor_c(r)` to be defined for
exactly the one
source leaf at the fixed path and writes that internally derived `q` into A1
envelope field 4 as `anchor(E)`.  An `OlderPublic` principal leaf is a checked
schema false, not an infrastructure abort.  Pairwise anchoring then requires
`q=o`, and

```text
sel_(ord,c)(nf(E)) = o
  iff origin_(nf(E))(root_c) = {o}.
```

There is no fallback, runtime branch, or first-member choice. An older origin
is a typed non-match. Missing replay or incomplete provenance yields no
disposition.

### 2.5 Complete realization predicate

The A1 realization judgment is a predicate on a completed envelope, not on its
raw code. To keep that type exact, A3-O freezes a deterministic build function
and then defines the predicate by replay. Write `kappa` for the exact binding
that the complete A4 repair chain must close over the profile, parents,
resources, and encodings. The following are definition-closed
templates; they mint no factual envelope until the later executable verifier
successfully applies them:

```text
BuildDerive^kappa_c(H,r)
  : (Sigma E:RawCapabilityEnvelopeV1. BuiltEnvelope(E))
      + False(FalseReasonV1) + Abort(AbortReasonV1)

Derive_(ord,c)^kappa(H,E) iff
  BuildDerive^kappa_c(H,raw_code(E))=BuiltEnvelope(E).
```

`raw_code(E)` is the canonical tagged subrecord projected from A1 field 11,
not an eighteenth envelope field, so checking `Derive` performs one exact
replay and no existential search.  `BuildDerive` returns `BuiltEnvelope(E)`
exactly when all of the following hold:

1. exact A1/A2/JG parent replay;
2. membership and successful deterministic decoding in `RawCode_c`;
3. all stated carrier formation and compatibility premises;
4. all internally generated JG2b2b1 substitutions and lifts;
5. independent raw-output checking against the schema output;
6. complete normalization/support evidence and the A2-O empty-rule ordinary
   `Primitive` certificate for every normalized public field;
7. the schema's unique `Current(q)` principal-root condition, with `q` written
   as the envelope anchor; and
8. deterministic construction of every exact theorem endpoint in section 5.

No theorem endpoint is assumed equal by this judgment. Later JG2b2b3a--c must
prove generic substitution and instantiate the constructor subjects.

## 3. The seven schemas

### 3.1 Formation

The raw source product is exactly `RawCode_Formation` of section 2.3. Its only
components are `Gamma:CtxCode` and `A:TelCode+(Gamma)`; `Here(Gamma)`, the
output record, evidence, and theorem endpoints are derived.

The dependent instance is

```text
Gamma ctx
A in PI(Gamma), |A| > 0
--------------------------------------------------------- Formation
Here(Gamma)  |->  A in PI(Gamma).
```

`A` is an output index extracted from a nonempty, dependency-ordered,
`V_b`-backed public result presentation. Only its fixed principal field must
have current-birth origin `{o}`. It is not an additional nominal input. The
raw result must replay every prefix `TypeFormation` judgment.
Nominal erasure is exactly

```text
PublicContext -> PublicInterface.
```

The principal position is the declaration-type root sourcing the oldest
output field `A_0`:

```text
root_Formation = FormationOldestOutputType
path(root_Formation) = Output.Interface.Field(0).TypeSource.
```

For `theta:Delta->Gamma`, the naturality subject has endpoints

```text
L_Form(theta,E) = nf(Out(reindex_theta(E)))
R_Form(theta,E) = nf(A[theta])
```

in `PI(Delta)`, with all prefix-formation evidence independently replayed.

### 3.2 Abstraction

The raw source product is exactly `RawCode_Abstraction`: `Xi:CtxCode`, one
bounded cut ordinal, and `B:TelCode+(Xi)`. `Gamma` and `A` are derived from the
cut; no second bound-telescope source is supplied.

For every exact nonempty suffix split `Xi = Gamma.A`, the dependent instance
is

```text
Gamma ctx
A in PI(Gamma), |A| > 0
B in PI(Gamma.A), |B| > 0
--------------------------------------------------------- Abstraction
(Here(Gamma.A), B)  |->  Fam(Gamma,A,B).
```

The verifier enumerates every integer `cut` with `0 <= cut < |Xi|`, sets
`Gamma=prefix_cut(Xi)` and `A=suffix_cut(Xi)`, and thereby permits an empty
`Gamma` but never an empty `A`. It does not prefer a shortest, longest, or
observed-success split. Distinct normalized splits remain distinct candidate
envelopes unless `approx_env,Abstraction` identifies their complete indexed
records and principal lineage. Nominal erasure is
exactly

```text
(PublicContext, PublicInterface) -> InterfaceFamily.
```

The principal position is the declaration-type root sourcing the oldest
family-result field `B_0`:

```text
root_Abstraction = AbstractionOldestFamilyType
path(root_Abstraction) = Output.Family.Result.Field(0).TypeSource.
```

For `theta:Delta->Gamma`, set `A'=A[theta]` and
`B'=B[theta^A]`. The naturality endpoints are

```text
L_Abs(theta,E) = nf(theta^* Fam(Gamma,A,B))
R_Abs(theta,E) = nf(Fam(Delta,A',B')).
```

### 3.3 Aggregation

The raw source product is exactly `RawCode_Aggregation`: `Gamma:CtxCode`, one
`FamilyCode(Gamma)=(A,B)`, and `C:TelCode+(Gamma)`. The aggregate output and
all evidence are derived.

The dependent instance is the target-neutral structural operation

```text
A in PI(Gamma)
B in PI(Gamma.A)
|A.B| > 0
C_raw in PI(Gamma), nf(C_raw) = nf(A.B)
--------------------------------------------------------- Aggregation
Fam(Gamma,A,B)  |->  A.B in PI(Gamma).
```

The equality is prefix-wise A2-O normalized telescope equality after
independent formation of `C_raw`; it is not a caller expected output.
Nominal erasure is exactly

```text
InterfaceFamily -> PublicInterface.
```

The principal position is the declaration-type root sourcing the oldest raw
output field:

```text
root_Aggregation = AggregationOldestRawOutputType
path(root_Aggregation) = Output.AggregatedInterface.Field(0).TypeSource.
```

For `theta:Delta->Gamma`, the naturality endpoints are

```text
L_Agg(theta,E) = nf((A.B)[theta])
R_Agg(theta,E) = nf(A[theta].B[theta^A]).
```

The later theorem is substitution compatibility of dependent telescope
concatenation. This schema selects no dependent type former.

### 3.4 Transport

The raw source product is exactly `RawCode_Transport`: decoded domain and
codomain contexts, source-backed substitution images, source interface, and
raw transported interface. The checked JG2b2b1 substitution record is derived.

The dependent instance is

```text
theta : Delta -> Gamma
A in PI(Gamma), |A| > 0
C_raw in PI(Delta), nf(C_raw) = nf(A[theta])
--------------------------------------------------------- Transport
(theta,A)  |->  A[theta] in PI(Delta).
```

The substitution is reconstructed and checked by the JG2b2b1 protocol; no
particular-substitution token is supplied. Nominal erasure is exactly

```text
(Substitution, PublicInterface) -> PublicInterface.
```

The principal position is

```text
root_Transport = TransportOldestRawOutputType
path(root_Transport) = Output.TransportedInterface.Field(0).TypeSource.
```

Because `Substitution` is bi-indexed, the one theorem-subject record contains
the exact two-whisker quartet. For
`sigma:Gamma->Omega`, `theta:Delta->Gamma`, `rho:Xi->Delta`, and
`Z in PI(Omega)` with `|Z|>0`, all four entries are independently formed in
`PI(Xi)`:

```text
T0 = nf(((Z[sigma])[theta])[rho])
T1 = nf((Z[sigma])[theta.rho])
T2 = nf(Z[sigma.(theta.rho)])
T3 = nf(Z[(sigma.theta).rho]).
```

The frozen subject contains the typed comparison edges

```text
T0 approx_ord T1,
T1 approx_ord T2,
T2 approx_ord T3,
```

where the final edge is the explicit alternate-parenthesization/interchange
edge. Setting `sigma` or `rho` to identity yields the codomain- and
domain-whiskering faces. No edge is proved by A3-O.

### 3.5 Comparison

The raw source product is exactly `RawCode_Comparison`: `Gamma:CtxCode` and
the two ordered nonempty telescope codes `A,B`. The canonical comparison replay
is derived; it is not supplied by the raw code.

The dependent instance is

```text
A,B in PI(Gamma), |A|=|B|>0
the separately checked normalized dependent telescope bytes agree
q(A,B) = the canonical KernelNormalizedTypeEquality witness
--------------------------------------------------------- Comparison
(A,B)  |->  q(A,B) in Cmp(Gamma,A,B).
```

Endpoint order is retained. There is no path, equivalence, transport square,
univalence, proof irrelevance, or cubical-depth claim. Nominal erasure is
exactly

```text
(PublicInterface, PublicInterface) -> ComparisonWitness.
```

The comparison output has no fabricated kernel proof term. Its checked output
realization is the joint replay record, whose principal position is the
declaration-type root sourcing the oldest field of the ordered left endpoint:

```text
root_Comparison = ComparisonOldestLeftType
path(root_Comparison) = Output.Comparison.Left.Field(0).TypeSource.
```

That field must have a unique current-birth origin. The right endpoint and all
fieldwise equality receipts remain in the output record; the selector does not
erase them.

For `theta:Delta->Gamma`, the naturality endpoints are

```text
L_Cmp(theta,E) = nf(q(A,B)[theta])
R_Cmp(theta,E) = nf(q(A[theta],B[theta])).
```

Equality is canonical A2-O comparison-witness identity after both endpoint
replays.

### 3.6 DemandCompiler

The raw source product is exactly `RawCode_DemandCompiler`: `Gamma`, the
source-backed grammar and scheme codecs, and the complete activation
`SecCode`. `Spec(S,a)` and its replay are derived.

Let `G=(E,O)`, `S=(X,ports,contracts)`, require `|X|>0`, and require at least
one port or contract. The dependent instance is

```text
a in Sec_Sigma(Gamma.E,X), with every component backed by V_b
L = Spec(S,a)
--------------------------------------------------------- DemandCompiler
(G,S)  |->  L in LiveDemand[Sigma,Gamma,G,S,a].
```

The activation `a` is a constructor-realization/output index, not a third
nominal input and not caller-provided evidence. The verifier rebuilds every
demanded output, dependency edge, operation anchor, and specialized contract.
Nominal erasure is exactly

```text
(SealedPublicGrammar, DemandScheme) -> LiveDemand.
```

The principal position is the term root sourcing the oldest activation image:

```text
root_DemandCompiler = DemandCompilerOldestActivationImage
path(root_DemandCompiler) = Output.LiveDemand.Activation.Image(0).TermSource.
```

Only this fixed oldest activation image must have singleton current-birth
origin `{o}`; later images may lawfully use older opaque public exports.

For `theta:Delta->Gamma`, let `G'=G[theta]`, `S'=S[theta]`, and
`a'=a[theta^E]`. The naturality endpoints are

```text
L_DC(theta,E) = nf(Spec(S,a)[theta])
R_DC(theta,E) = nf(Spec(S',a')).
```

This is positive activation only. Present derivability is never queried by
the schema.

### 3.7 DischargeTransformer

The raw source product is exactly `RawCode_DischargeTransformer`: decoded
domain/codomain contexts, substitution images, `G`, `S`, activation `a`,
source discharge `d`, and raw transformed discharge `d'_raw`. All reindexed
indices are derived.

Let `theta:Delta->Gamma`, `G=(E,O)`, `S=(X,ports,contracts)`,
`a in Sec_Sigma(Gamma.E,X)`, `L=Spec(S,a)`, and
`d in Dch(Gamma,G,S,L)`. Require `|L.DemandedTelescope|>0`. Define

```text
G' = theta^*G
S' = theta^*S
a' = a[theta^E]
L' = Spec(S',a').
```

The dependent instance is

```text
d'_raw in Dch(Delta,G',S',L')
nf(d'_raw) = nf(theta^*d) with every dependent index and contract replayed
--------------------------------------------------------- DischargeTransformer
(theta,d)  |->  d[theta^E].
```

Nominal erasure is exactly

```text
(Substitution, Discharge) -> Discharge.
```

The single principal position is:

```text
root_DischargeTransformer = DischargeTransformerOldestRawOutputImage
path(root_DischargeTransformer) =
  Output.Discharge.Section.Image(0).TermSource.
```

There is no contract-only fallback or runtime selector branch.

The exact two-whisker theorem subject binds
`sigma:Gamma->Omega`, `theta:Delta->Gamma`, `rho:Xi->Delta`, a grammar `K`
over `Omega`, scheme `T` over `K`, activation `b`,
`M=Spec(T,b)` with `|M.DemandedTelescope|>0`, and
`e in Dch(Omega,K,T,M)`. It independently constructs the fully indexed
quartet in the final context `Xi`:

```text
D0 = nf(rho^*(theta^*(sigma^*e)))
D1 = nf((theta.rho)^*(sigma^*e))
D2 = nf((sigma.(theta.rho))^*e)
D3 = nf(((sigma.theta).rho)^*e).
```

The frozen subject contains the typed discharge-equality edges

```text
D0 approx_ord D1,
D1 approx_ord D2,
D2 approx_ord D3,
```

and retains every intermediate reindexed `K/T/b/M` index, section type,
contract, and equality receipt. The last edge is the explicit
interchange/alternate-parenthesization edge. No edge is proved here.

## 4. Closed candidate universes and deterministic classification

For each internally selected `c`, the raw universe is exactly `RawCode_c` of
section 2.3. There are no extension slots. Derived output records, provenance,
evidence, and theorem descriptors are constructed by the deterministic
functions below, not enumerated as raw input.

### 4.1 Canonical built records and public-field paths

`Built_c` is not an implementation-chosen map.  It is the following tagged
sum of ordered records.  Every row has the common prefix
`(tag,raw_code,decoded,anchor,indices,nominal_inputs,nominal_output,
raw_realization)`; the table fixes the final four components after `anchor`.
Raw and normalized forms and every decoding receipt remain inside `decoded`.

| `c` | `indices` | `nominal_inputs` | `nominal_output` | `raw_realization` |
| --- | --- | --- | --- | --- |
| `Formation` | `(Gamma)` | `(Here(Gamma))` | `A` | `A_raw` |
| `Abstraction` | `(Gamma,A,Gamma.A)` | `(Here(Gamma.A),B)` | `Fam(Gamma,A,B)` | `(Xi_raw,cut,B_raw)` |
| `Aggregation` | `(Gamma,A,B)` | `(Fam(Gamma,A,B))` | `A.B` | `C_raw` |
| `Transport` | `(Delta,Gamma)` | `(theta,A)` | `A[theta]` | `C_raw` |
| `Comparison` | `(Gamma,A,B)` | `(A,B)` | `q(A,B)` | `(A_raw,B_raw)` |
| `DemandCompiler` | `(Gamma,G,S,a)` | `(G,S)` | `Spec(S,a)` | `a_raw` |
| `DischargeTransformer` | `(Delta,Gamma,G,S,a,L)` | `(theta,d)` | `theta^*d` | `d_raw` |

Here a `_raw` name denotes the exact decoded source-backed presentation; the
unnamed counterpart is the independently checked normalized carrier.  The
common `anchor` is the `q` derived from the unique principal `Current(q)` leaf
of section 2.4.  No field in this record is supplied after decoding.

Define `PublicLeaves(path,value)` by the following exhaustive tagged preorder:

- a context or telescope emits `Field(i).Type` in oldest-first order;
- a section or substitution emits `Image(i).Term` in oldest-codomain-first
  order;
- a family emits its index telescope, then result telescope;
- a comparison emits its left endpoint, then right endpoint;
- a grammar emits export fields, then for each operation ordinal its parameter
  fields and result fields;
- a scheme emits trigger fields, then each port's argument images and derived
  output fields, then canonically sorted contracts in `Type,Left,Right` order;
- a live demand emits activation images, demanded-output fields, then
  specialized contracts in `Type,Left,Right` order; and
- a discharge emits section images, then instantiated contracts in
  `Type,Left,Right` order.

For records and tuples, traversal follows declared field order.  Numeric tags,
receipts, and proofs emit no public leaf.  The exact per-constructor vector is

```text
EnvelopeFieldPaths_c(b)
  = PublicLeaves(Indices,b.indices)
 ++ PublicLeaves(Inputs,b.nominal_inputs)
 ++ PublicLeaves(Output,b.nominal_output)
 ++ PublicLeaves(RawRealization,b.raw_realization)

Fields_c(b)
  = map (lambda p. (p,nf(project_p(b)))) (EnvelopeFieldPaths_c(b)).
```

Every member is a full `EnvelopeFieldPathV1`; its typed root is part of the
path. Therefore the same mathematical
field retained in two index/input/output/raw-record positions occurs twice
with two distinct paths; traversal never semantically deduplicates it.

For every emitted path `p`, `FieldLineageV1(p)` is the ordered vector of
triples `(raw_leaf_path:RawFieldPathV1,source:SourceLeafV1,derivation_edge)`
obtained by replaying the fixed `Decode_c;Build_c` dependency DAG backwards
from `p`. A direct raw leaf has its singleton triple. Telescope concatenation preserves left-then-right
order; substitution, specialization, comparison, and discharge follow their
displayed dependency order and retain every contributing edge.  Repeated
edges remain repeated in this lineage vector; only the later support-set
projection sorts and deduplicates irreducible identities.  Thus lineage is a
deterministic replay record, never inferred by searching normalized syntax.

The schema-specific output-source positions used for the separate A1 current-
birth/output footprint are exactly:

```text
Formation             RawRealization.A_raw.Field(i)
Abstraction           RawRealization.B_raw.Field(i)
Aggregation           RawRealization.C_raw.Field(i)
Transport             RawRealization.C_raw.Field(i)
Comparison            RawRealization.A_raw.Field(i), then B_raw.Field(i)
DemandCompiler        RawRealization.a_raw.Image(i)
DischargeTransformer RawRealization.d_raw.Image(i).
```

`OutputUse_c(r)` exhaustively traverses, by JG2b2b0, every `Current(q)` term
selected at those paths.  `OutputLineage_c(r)` separately retains the ordered
`(path,q)` vector before set projection.  Define

```text
CurrentOutput_c(r) = sort_unique(Cl_b(OutputUse_c(r)))
Implementation_c(r) = sort_unique(Cl_b({anchor_c(r)} union Use_b(r)))
PriorSupport_c(r) = Old_b(r).
```

For every successful build `E`, the principal condition makes
`CurrentOutput_c(r)` nonempty and places `anchor_c(r)=anchor(E)` in it.  These
three canonically ordered footprints are separate; none may be substituted for
another.

### 4.2 Exact ordinary structural-support construction

`Evidence_c.support` is exactly `OrdinarySupportV1`; it is not an
implementation callback. Before support, define the pure projection

```text
SubjectRootProjection_c(built)
  : SubjectRootProjectionV1.
```

Its candidate root vector is the following tagged concatenation, in this
order:

```text
SupportRoots_c(built) =
    PublicLeaves(Indices,built.indices)
 ++ PublicLeaves(Inputs,built.nominal_inputs)
 ++ PublicLeaves(Output,built.nominal_output)
 ++ PublicLeaves(RawRealization,built.raw_realization)
 ++ SourceLeaves(RawCode,built.raw_code)
 ++ ObligationRoots(built)          -- A1 field 12 order
 ++ SubjectProjectionRoots(SubjectRootProjection_c(built)).
```

`SourceLeaves` is raw-field preorder.  `ObligationRoots` follows the eight
premises of section 2.5 and each schema's displayed premise order.
`SubjectRootProjection_c` is deterministically constructed from the built
value before support. It contains the exact instantiated endpoint literals in
identity, composition, selector, `Nat_c`, and extra-coherence order, but no
field-16 wrapper, support object, quotient key, or action image. Generic
universally bound metavariables are declarations, not dependencies. Field 16
later retains the full subject DAG and the byte-identical projection. Evidence
fields 13--15 and commitments are excluded, preventing recursive support.

Each root is traversed by the declared carrier-field order of A2-O. Contexts,
telescopes, sections, substitutions, operations, ports, contracts, and
discharges use the orders frozen in `PublicLeaves`.  A kernel term uses the
JG2b2b0 node and child-edge order.  At each record or term node, dependency
slots are emitted before descending to child terms in this finite enum order:

```text
RawDependencyEdgeV1 ::=
  SourceLeaf | LocalBinderUse | GlobalUse | TelescopePrefix
| SectionExpectedType | SubstitutionImage | FamilyIndex
| GrammarExport | OperationAnchor | OperationParameter | OperationResult
| SchemeTrigger | PortArgument | PortDerivedOutput | ActivationImage
| ContractType | ContractLeft | ContractRight | DischargeImage
| ObligationPremise | TheoremEndpoint | TermChild(JG2b2b0Edge)
| RecordReconstruction(ReconstructionTagV1)
| NormalizationStep(NormalizationRuleV1)
| ClosureAncestor | ClosureBinderSibling | ClosureSameBirth
| ActionIntroduced | ActionArgument.
```

The two nested enums are also closed:

```text
ReconstructionTagV1 ::=
  ExpectedType | CaptureSafeSubstitution | IteratedBinderLift
| TelescopeConcatenation | ComparisonReplay | GrammarReindex
| SchemeSpecialization | DischargeContractReplay.

NormalizationRuleV1 ::=
  BetaApply | PermittedGlobal(GlobalId)
| FirstPairProjection | SecondPairProjection.

ExactReferentV1 ::=
  CurrentOccurrence(OccurrenceId14V1)
| OlderExport(ExportIdV1,PublicFieldPathV1::DeclarationField)
| KernelGlobal(GlobalId)
| Binder(BinderKeyV1)
| RecordSlot(FullLocalPathV1)
| OperationSlot(grammar_path:FullLocalPathV1,
                operation_ordinal:u32,
                parameter_or_result:ParameterOrResultV1,
                field_ordinal:u32)
| NormalizationNode(FullLocalPathV1,NormalizationRuleV1)
| FormalAction(FormalActionIdV1).

FormalActionIdV1 ::=
  FormalBinder(subject_path:SubjectPathV1,binder_ordinal:u32)
| FormalGlobal(subject_path:SubjectPathV1,global_ordinal:u32).

ExpectedJudgmentV1 ::=
  TypeFormation
| HasType(normalized_type)
| DefinitionallyEqualAt(normalized_type)
| ContextPrefix(normalized_context_id,prefix_ordinal)
| CarrierSlot(carrier_index:CarrierIndexValueV1,field_ordinal:u32)
| OperationOrdinal(grammar_path:FullLocalPathV1,operation_ordinal:u32)
| StructuralOccurrence(OccurrenceId14V1)
| NormalizationSubject(NormalizationRuleV1).
```

Every displayed base sum uses the displayed variant order;
`ParameterOrResult` is
`Parameter` then `Result`, and carrier tags use the frozen A2 nine-sort order.
In the base display, `normalized_context_id` is a legacy metavariable name for
the complete canonical `NormalizedContextV1` value, not a digest, handle, or
arbitrary byte identifier; `prefix_ordinal` is `u32`. A3-C1 therefore leaves
the payload bytes of tag 3 unchanged.
`CarrierIndexValueV1` is the full closed nine-carrier index sum supplied by
A4-R2b; arbitrary index bytes are forbidden. `FormalBinder` and `FormalGlobal`
are positions in `SubjectSyntaxV1`. A concrete bounded action term never uses
`FormalGlobal` as a substitute for a kernel global: every `Term::Global(g)`
must check under `Sigma(H,o)` and resolve to an exact current or older owner.
None of these records admits a caller string or fabricated history identity.

In particular, `TypeFormation` does not fabricate a universe or an expected
type. Structural and ordinal edges use their corresponding non-typing tag.
`exact_referent` and `expected_judgment` are edge-indexed: an edge whose tag
cannot admit the displayed referent/judgment variant is a checked schema
false.

Every emitted record has the exact shape

```text
RawDependencyV1 =
  (raw_ordinal, support_root_tag, full_local_path, edge_tag,
   exact_referent:ExactReferentV1,
   expected_judgment:ExpectedJudgmentV1,
   owner_resolution_path).
```

`raw_ordinal` is contiguous in traversal order.  Dependent-prefix references,
operation anchors, derived port outputs, contracts, and obligations emit their
record edge even when their underlying term syntax contains no `Global` or
`Var` node. In the post-R2b concrete schema, A3-C1 gives such a pure typed
record `StructuralRecord` with its exact nonzero schema `TypeIdV1`, and a
required static operation `ConstructionRule` with its complete
`DefinitionRulePathV1`. Repeated uses remain repeated raw records.

Owner resolution is deterministic:

1. `Current(q)` resolves from `H` to the exact fourteen-field occurrence
   identity and declaration owner.  Its owner is
   `CandidateField(EnvelopeFieldPathV1)` when reached through an emitted public field's
   `FieldLineageV1`, otherwise `AuxiliaryRoot(support_root_tag,full_local_path)`
   for a raw context locator, obligation-only source, or other
   implementation-only use. It is not pretended to be an already public field
   of `H`.
2. Every concrete `Global(g)` in a candidate or bounded action first checks
   under `Sigma(H,o)`. If `Birth(g)=b`, it resolves to the exact same-birth
   declaration-type occurrence; an actual permitted unfold additionally
   resolves to the exact body occurrence. If `Birth(g)<b`, it resolves through
   the unique `Pub_<b(H)` `DeclarationField` export. A later, missing,
   ambiguous, nonpublic, or private-body target is a candidate/support false.
   At a caller action locus, an unknown global is the closed action-false
   variant; in an internally rebuilt trace it is an invariant abort. No
   `ExternalGlobal` or formal owner is constructed.
3. `Var(i)` resolves through its retained `BinderKeyV1`. A historical key
   points to its JG2b2b0 binder-entering occurrence; a synthetic key points to
   its unique earlier raw telescope field and follows that field's
   `FieldLineageV1`. In an action image only,
   `BinderKeyV1::FormalBinder(subject_path,binder_ordinal)` resolves to
   `FormalActionOwner(FormalBinder(subject_path,binder_ordinal))` and is
   excluded from every history footprint. The same key in a candidate is
   false.
4. An interface/operation/port/contract/obligation dependency follows its
   exact record ordinal and `FieldLineageV1` edges until it reaches a current
   occurrence or older public field.
5. A `RecordSlot`, `OperationSlot`, or `NormalizationNode` with no underlying
   term owner terminates at an exact `StructuralOwner`; it is covered and
   quotiented but does not enter any history footprint.
6. Only an action image may terminate at
   `FormalActionOwner(FormalActionIdV1)`; a raw-code candidate with an
   unresolved formal or private referent is false.

The retained owner sum is therefore exactly

```text
SupportOwnerV1 ::=
  CurrentOwner(OccurrenceId14V1,
               CandidateField(EnvelopeFieldPathV1)
                 | AuxiliaryRoot(FullLocalPathV1))
| OlderOwner(ExportIdV1,PublicFieldPathV1::DeclarationField)
| StructuralOwner(FullLocalPathV1,ExactReferentV1)
| FormalActionOwner(FormalActionIdV1).
```

Resolution is well founded by the lexicographic measure
`(birth ordinal, declaration ordinal, carrier-prefix ordinal,
derivation-DAG height)`: history globals point strictly backward, dependent
records point to earlier prefixes, and derived outputs point into the finite
acyclic `Decode_c;Build_c` DAG.

The A4-R3 repair must close the exact `FalseReasonV1` and `AbortReasonV1`
taxonomy. The following semantic partition constrains that closure. A
syntactically valid but profile-disallowed raw referent--a
nonmember or ambiguous older-export request, private-body request, formal
dependency in a candidate, or nondecreasing/cyclic candidate edge--is the
corresponding `FalseReasonV1::Support` variant. Failure to replay a parent
identity that the already verified `H` claims to own, arithmetic/allocation or
resource exhaustion, or inability to complete the fixed traversal/checker is
`Abort(AbortReasonV1)`. Neither case emits partial support, and only `Abort`
suppresses the entire pair disposition. Kernel logical errors are false only
at a candidate/action judgment locus; the same error during normalization,
deterministic reconstruction, or verified parent/history replay is aborting.

After direct resolution, current occurrence identities are expanded by
`Cl_b`.  The added records are emitted in canonical occurrence-identity order
with respectively `ClosureAncestor`, `ClosureBinderSibling`, or
`ClosureSameBirth` edges and the exact predecessor record that forced each
addition.  The candidate build requires

```text
current_projection(OrdinarySupportV1) = Implementation_c(r)
older_projection(OrdinarySupportV1)   = PriorSupport_c(r),
output_root_projection                = CurrentOutput_c(r).
```

`current_projection` erases owner/judgment data, then sorts and deduplicates
exact current occurrence identities; `older_projection` analogously returns
exact older export identities.  The following output projection applies the
same erasure only after its stricter root/path filter.

`output_root_projection` is **not** the projection of every `Output`-tagged
root. It is exactly the current-occurrence projection reached from the seven
schema-specific `OutputLineage_c(r)` raw source paths of section 4.1, followed
by their deterministic `OutputUse_c` traversal and `Cl_b` closure. For an
action image it uses the reindexed counterparts of only those paths plus
exact action-introduced current occurrences inside those slots. Formal action
dependencies retain their output-root tags in structural support but never
enter the exact-history `CurrentOutput` footprint. Every other dependency of a
derived nominal output remains in `Implementation`/structural support but does
not enter `CurrentOutput`.

The irreducible support key is one of

```text
CurrentSupport(exact_occurrence_id,
               CandidateField(EnvelopeFieldPathV1)
                 | AuxiliaryRoot(FullLocalPathV1),
               ExpectedJudgmentV1)
OlderSupport(exact_export_id, PublicFieldPathV1::DeclarationField,
             ExpectedJudgmentV1)
StructuralSupport(FullLocalPathV1,ExactReferentV1,
                  ExpectedJudgmentV1)
FormalActionSupport(FormalActionIdV1, ExpectedJudgmentV1).
```

The fourth form is unavailable to candidates. Two raw dependencies are in the
same support class iff these complete keys are byte-equal; distinct exact
owners never collapse merely because terms normalize alike.  Classes are
sorted lexicographically by `(variant tag, complete tagged-owner/referent
bytes, expected-judgment bytes)`. `raw_to_class[i]` is the sorted-class ordinal
of raw record `i`, and the canonical support set is the resulting sorted,
duplicate-free key vector.

`StructuralSupport` and `FormalActionSupport` are evidence-only classes. They
are excluded from current/older history projections and can never be counted
as paid, prior-live-output, or generative-provenance support.

Coverage uses two passes.  A shape-only pass first enumerates every
`(root,path,expected edge slot)` without resolving a referent.  The resolution
pass must emit exactly one contiguous raw record for every slot, no extra
record, plus every deterministically forced closure record.  The certificate
retains both vectors, their per-root node/slot counts, the closure worklist in
canonical order, the full owner map, `raw_to_class`, and an all-resolved
bit-vector.  Equality of the two slot vectors and exhaustion of the closure
worklist are checked structurally, not by a digest alone.

Finally `OrdinarySupportV1` retains, but does not prove, preservation subjects
whose endpoints are pure `SupportProjectionV1` values after normalization,
admissible history-identifier renaming, and final normalized action. It never
embeds normalization evidence, a containing support wrapper, a quotient key,
or an action image. For an action image, support roots use the final canonical
payload, the base candidate's byte-identical raw source leaves, final
obligations, the final `SubjectRootProjectionV1`, and appended
`ActionArgument` roots. It runs the same traversal, owner resolution,
quotient, sorting, and two-pass coverage algorithm from scratch.  For the
empty identity action the appended segment is empty and every root tag/path,
raw record, owner, quotient entry, coverage vector, and canonical support key
is required byte-equal to the base candidate's support evidence.

### 4.3 Deterministic envelope assembly

The evaluator is the following deterministic partial pipeline; these function
names and codomains are part of the freeze and are parameterized by the future
A4 binding `kappa`:

```text
Decode_c(H,r)
  : RawCode_c -> Decoded_c + False(FalseReasonV1) + Abort(AbortReasonV1)

Build_c(H,decoded)
  : Decoded_c -> Built_c + False(FalseReasonV1) + Abort(AbortReasonV1)

Fields_c(built)
  : finite ordered vector of (EnvelopeFieldPathV1,normalized public field)

SubjectRootProjection_c(built)
  : SubjectRootProjectionV1

Evidence_c(H,built,subject_projection)
  : (complete normalization evidence over SupportProjectionSubjectV1,
     OrdinarySupportV1(H,built,subject_projection),
     for every p in Fields_c,
       Primitive(CompleteNoReplacementCertificate(
         ordinary-empty-replacement-transcript,p)))

Subjects_c(built,subject_projection)
  : (SubjectSyntaxV1, byte-identical SubjectRootProjectionV1)

Envelope^kappa_c(H,built)
  : RawCapabilityEnvelopeV1 = assemble the exact A1 field map below

BuildDerive^kappa_c(H,r)
  : (Sigma E:RawCapabilityEnvelopeV1. BuiltEnvelope(E))
      + False(FalseReasonV1) + Abort(AbortReasonV1).
```

`Decode_c` applies exactly the partial decoders of section 2 in field order.
`Build_c` applies exactly the displayed constructor rule of section 3 and no
other rule and returns exactly the corresponding table row above. `Fields_c`
is exactly the `EnvelopeFieldPaths_c` projection, not an informal carrier traversal;
evidence records themselves are not public fields.
`Evidence_c` never calls `Decode_c`, `Build_c`, or the envelope classifier
recursively: the A2-O ordinary replacement grammar has zero rules, so its
complete negative certificate is the unique empty enumeration. Its support
component is exactly the section-4.2 algorithm, including its two-pass
coverage certificate; no alternative support traversal is admitted.
`Subjects_c` is the topologically ordered `SubjectSyntaxV1` DAG fixed by
A4-R2a. Its literals are restricted to typed core, carrier, occurrence,
formal-action-argument, and pure support-function projections. It contains no
full envelope, support wrapper, quotient/final key, action trace, action image,
pair transcript, or disposition. Its retained root projection must be
byte-identical to the projection already consumed by support. Its constructor
is fixed by `c`; it does not enumerate substitutions, ask for a proof, or
consume a caller endpoint.

Within one complete raw leaf, first failure is checked in the exact order
`Decode_c` fields; `Build_c` premises in displayed order; `Fields_c`;
`SubjectRootProjection_c`; normalization; structural support; empty-grammar
primitive certificates; `Subjects_c`; `Envelope^kappa_c`; `Anchor`; then
`Compat`. Profile/parent replay and raw-universe preflight precede the first
leaf. This order fixes both the closed failure receipt and deterministic
logical resource transcript.

`Envelope^kappa_c` maps the A1 fields 1--17 without any open structural or
ordering choice:

| A1 field | Exact A3-O content |
| --- | --- |
| 1 | `kappa`'s protocol/schema/profile/parent versions and constructor tag `c` |
| 2 | `kappa.parents` in A1's fixed parent order |
| 3 | the owned complete-through-head binding `H` |
| 4 | the exact fourteen-field identity and normalized structural identity replayed for `anchor(E)` |
| 5 | `Implementation_c(r)` with its ordered exact occurrence identities |
| 6 | `PriorSupport_c(r)` followed separately by `CurrentOutput_c(r)` |
| 7 | the exact predecessor/birth/successor boundaries and local binder contexts replayed from `H` for fields 4--6 |
| 8 | the tagged `Built_c` constructor subject |
| 9 | `built.indices` and `built.nominal_inputs` in declared order |
| 10 | `built.nominal_output` with all dependent indices and values |
| 11 | `raw_code`, `decoded`, and `built.raw_realization` |
| 12 | the ordered schema premises and all internally derived kernel/profile/bridge obligations |
| 13 | complete `Evidence_c.normalization` over `SupportProjectionSubjectV1` |
| 14 | complete `Evidence_c.support`, including `FieldLineageV1` |
| 15 | the `Fields_c`-indexed disposition vector |
| 16 | `(SubjectSyntaxV1,byte-identical SubjectRootProjectionV1)`, including every identity/composition and naturality/coherence subject |
| 17 | `encode_kappa(fields 1--16)` and the separate exact-evidence commitment |

The completed A4 repair record must bind only the version values, exact parents,
resource policy, and canonical encoder admitted by `kappa`; it may not add, remove,
reorder, or
reinterpret an A1 field, a `Built_c` component, or an `EnvelopeFieldPathV1` entry.

Define the three-valued evaluator

```text
Eval^kappa_c(H,o,r) =
  Abort(e)  if BuildDerive^kappa_c(H,r)=Abort(e);
  False(q)  if BuildDerive^kappa_c(H,r)=False(q);
  Match(E)  if BuildDerive^kappa_c(H,r)=BuiltEnvelope(E),
               Derive_(ord,c)^kappa(H,E),
               Anchor_(ord,c)^kappa(H,E,o), and
               Compat_(ord,c)^kappa(H,o,Implementation_c(r),r,E);
  False(FalseReasonV1::Anchor | FalseReasonV1::Compatibility)
    otherwise after a successful non-aborting build.
```

Thus `BuildDerive` is the deterministic
`Decode_c;Build_c;Fields_c;SubjectRootProjection_c;Evidence_c;Subjects_c;
Envelope^kappa_c` pipeline, while
the A1-typed `Derive(H,E)` is its exact replay predicate on the retained raw
code of `E`. Neither contains a search choice.

Carrier presentation equality remains `approx_ord`. The envelope quotient is
the strictly finer relation.  Let `QuotientKeyV1_c(E)` be the canonical
ordered record containing all of the following data from the normalized
envelope:

1. protocol, schema, profile, parent, and owned-history bindings;
2. every normalized carrier, index, input, output, public-interface, and
   implementation realization;
3. the normalized predecessor/birth/successor boundary and local-context
   records;
4. `PrincipalRootLineageV1` and `FieldLineageV1`, where the latter records the
   exact source occurrence and fixed schema path of every source-backed field
   or image leaf, not merely the principal leaf;
5. the complete canonically ordered implementation, current-birth/output,
   and prior-support footprints;
6. the canonical `EnvelopeSupportEvidenceV1` payload instantiated exactly by
   `OrdinarySupportV1`: every raw dependency
   and derivation edge, owner/public-field map, raw-to-irreducible quotient,
   normalized support set, and coverage/preservation subjects;
7. the ordered primitive-or-derived disposition vector for every normalized
   public field; and
8. every internally derived obligation and the exact identity, composition,
   reindexing, and naturality subjects.

Every retained theorem subject is `SubjectSyntaxV1` over the admitted
projection literals. No theorem subject contains `QuotientKeyV1_c`,
`FinalActionKeyV1`, an action trace, or an action image. `FinalActionKey_c`
below is mathematical notation for the quotient key returned by deterministic
action rebuild, not a subject node or literal.

All identifier-bearing entries in this key use exact identities.  All
set-like entries use their already specified canonical order.  Consequently
the definition below applies to both raw-code candidate envelopes and the
non-classifiable action images of section 5; an action image supplies its
recomputed `FinalActionKey`. `Derive` and `Cand` continue to range over
candidate envelopes only.

```text
E approx_(env,c) E' iff
  c=c',
  corresponding normalized term-bearing entries of QuotientKeyV1_c(E)
    and QuotientKeyV1_c(E') are approx_ord,
  and every remaining entry of those keys is byte-equal.
```

`PrincipalRootLineageV1` retains the `OriginPathV1` tag and exact source
occurrence identity before transitive support expansion. It is distinct from
the support set.  `FieldLineageV1` applies the same exact-source discipline to
all nonprincipal leaves.  Thus equal carrier normal forms cannot collapse
different anchors, nonprincipal provenance, implementation/current/prior
footprints, support quotients, or disposition vectors.

Normalization derivation traces, allocation/resource receipts, checker proof
objects, envelope bytes, and commitment encodings are retained in the
representative transcript but are not additional class keys.  Every
`approx_(env,c)` witness must instead replay both representatives and verify
that normalization transports every item in `QuotientKeyV1_c` to the common
key.  No field listed above may be discarded merely because its carrier term
normalizes to the same syntax.

For fixed `(kappa,H,o,c)`, enumerate every `r in RawCode_c`, evaluate
`Eval^kappa_c`, and
form

```text
Cand^kappa_ord(H,o,c)
  = { nf(E) | r in RawCode_c, Eval^kappa_c(H,o,r)=Match(E) }
      / approx_(env,c).
```

The source set, length bounds, slot products, footprint closure, checking
judgments, normalization, and quotient equality are all finite and decidable
relative to the fixed verified history and resource profile. The classifier
may emit an A1 pair disposition only after complete enumeration:

```text
0 classes  -> CertifiedNoMatch
1 class    -> UniquePositive
>1 classes -> TypedAmbiguity.
```

A `False` result excludes only that code. If any code returns `Abort`, or the
finite loop itself is incomplete, the whole pair classification emits no
disposition. Only a complete all-non-aborting loop may count the quotient.
The repaired A4-R2d/R3 coverage contract must order all raw codes by the structural ordinal above,
sorts quotient classes lexicographically by full `QuotientKeyV1_c` bytes,
sorts class members by raw ordinal, and chooses the least raw ordinal as the
canonical representative. It retains every equivalent member and a complete
outcome vector. Multiple raw matches in one quotient class therefore yield
`UniquePositive`, not ambiguity; digest inequality is never used as an
inequivalence witness. `CertifiedNoMatch` requires complete all-false coverage,
and `TypedAmbiguity` retains every full-key-distinct class. No early exit or
ambiguity tie-break is admitted.
One schema per constructor proves closed constructor coverage; it proves
neither that every occurrence matches nor that the seven schemas are mutually
exclusive.

## 5. Frozen selector and action subjects

A generic action does not return another raw-code candidate: arbitrary formal
substitution syntax need not lie in the finite source grammar.  A3-O therefore
uses the A1 action-image boundary and freezes

```text
CandidateEnvelope_c^kappa(H,o)
  = (Sigma r,E. Eval^kappa_c(H,o,r)=Match(E))

EnvelopeObject_c^kappa(H,o)
  ::= CandidateEnvelope(E)
    | ActionImage_c(base_candidate:CandidateEnvelope(E),
                    normalized_action, checked_action_trace).
```

Thus `CandidateEnvelope(E)` is a dependent refinement carrying the successful
`BuildDerive`, envelope-typed `Derive`, full A1 `Anchor`, and `Compat` witness
hidden in `Eval=Match(E)`; it is not a free wrapper around an arbitrary
`RawCapabilityEnvelopeV1`.  In this section `kappa,H,o,c` remain fixed and are
suppressed from constructor notation.

`ActionImage_c` is an opaque dependent constructor, not a free tuple. Its
private projections are definitionally

```text
anchor(ActionImage_c(B,a,t)) = sel(B)
(payload(ActionImage_c(B,a,t)), key(ActionImage_c(B,a,t)))
  = CanonicalActionResult_c(B,nf(a)).
```

Construction checks that `t` starts at `B`, has flattened action `nf(a)`, and
contains exactly the well-typed step/sequential endpoints used to form that
action; failure constructs no object.  The trace supplies no equality proof.
Thus malformed anchors, payloads, or keys do not inhabit `EnvelopeObject`.
At a caller-proposed action locus, a base/domain/codomain/arity/typing/square
failure must become the exact A4-R3 `FalseReasonV1::Action` variant. Failure of an
internally reconstructed trace, parent, normalization, resource account, or
canonical result is `Abort(AbortReasonV1)` and constructs no object. No caller
supplies a `checked_action_trace`.

`ActionImage` makes no `Derive` claim and is excluded from `Cand`.  The
selector, `nf`, and `approx_(env,c)` extend to `EnvelopeObject`.  The selector
reads the principal lineage retained inside `FinalActionKey`.
`nf(ActionImage_c(B,a,t))=ActionImage_c(B,nf(a),normalize_trace(t))`; its
dependent payload and key projections are thereby recomputed by
`CanonicalActionResult`, while the representative-only trace retains its
checked route. `normalize_trace` is the deterministic displayed-step order
with canonical endpoint encodings and no equality contraction; applying it
twice is literal identity. This operation is idempotent by construction.
Presentation comparison uses that
same final `QuotientKeyV1_c` shape.  The variant tag and proof/resource trace
are not class keys, so an identity image can be presentation-equivalent to its
base candidate.

`Base(CandidateEnvelope(E))=CandidateEnvelope(E)` and
`Base(ActionImage_c(B,...))=B`; thus `Base` returns the original candidate by
value through any nesting and requires no identity-to-content resolver.  The
base candidate's `kappa`-canonical identity is separately retained in the
action trace. `anchor(CandidateEnvelope(E))` is A1
field 4, `anchor(ActionImage_c(B,...))=sel(B)`, and in either case it equals
`sel(Env)`. `ActionTrace(q,Env)` commits to the unflattened
action and replay route.  The private `schema_payload` projection is exactly the displayed reindexed
carrier tuple for `c`; its complete field lineage is also retained in
`FinalActionKey`.  `ExtendActionNF(Env,q)` concatenates the action already
stored by `Env` (or `[]` for a candidate) with `ActionNF(q)`;
`ExtendSquareNF` is analogous. These accessors fix the common constructor
arity below.

For substitutions, `ActionNF(id)=[]`, a primitive formal substitution has the
singleton list `[theta]`, and `ActionNF(q1.q2)` is list concatenation; the final
term action is rebuilt in one fixed right-associated order.  For context
squares, `SquareNF` analogously flattens primitive squares, recomputes both
vertical composites, and constructs the pointwise pasted `eps` syntax in one
fixed left-to-right tree; identity is the empty list.  Acting on an existing
`ActionImage` concatenates its stored normalized action with the new one and
rebuilds directly from the original base candidate, never from an intermediate
key.

The single deterministic constructor

```text
CanonicalActionResult_c(BaseCandidate,normalized_action)
  : (canonical_schema_payload_c, FinalActionKey_c)

CanonicalActionResult_c(B,[])
  = (Payload_c(B),QuotientKeyV1_c(B)).
```

always rebuilds both components from the base candidate and the same flattened
action expression; the displayed empty case is literal and bypasses no
candidate check. For a nonempty action, `FinalActionKey_c(q,Env)` is its second projection at
`(Base(Env),ExtendActionNF(Env,q))` (or the square analogue); its first
projection is `CanonicalActionPayload_c(q,Env)`.  The constructor does all of
the following:

1. reindexes every carrier/index/output and raw realization by the unique A2
   recipe;
2. retains every historical principal and nonprincipal source-lineage record;
3. recomputes `EnvelopeFieldPaths_c`, normalization evidence, obligations, theorem
   endpoints (including the exact normalized-action parameter), and the
   empty-grammar `Primitive` vector from the final result;
4. exhaustively recomputes structural support from the final normalized
   realization, adding every dependency edge contributed by the action;
5. recomputes the three history footprints: an exactly replayed current
   occurrence enters `Implementation`, and enters `CurrentOutput` only through
   the reindexed `OutputLineage_c` source slots fixed in section 4.2; an exactly
   replayed older opaque export enters `PriorSupport`; each formal binder or
   subject-variable dependency remains a distinct
   `FormalActionSupport(FormalActionIdV1,ExpectedJudgmentV1)` class. Every
   concrete global instead resolves under `Sigma(H,o)` to an exact current or
   older owner; no `ExternalGlobal` support class exists; and
6. recomputes the raw-to-support quotient and canonical support set after
   those additions, while retaining both the original source-path lineage and
   a separate action-introduced lineage vector.

The current intermediate payload is used only to type-check the newly appended
action.  The sequential presentation that would result from acting on it is
retained as an identity/composition endpoint in `action_trace_commitment`; it
is never stored as the canonical payload.  Thus direct and iterated `Re` use
the same payload/key constructor before their equality is proved.
The key's instantiated naturality entry is the canonical `SubjectSyntaxV1`
built from core/action endpoint projections for the one canonical composite;
a newly appended stepwise endpoint remains representative trace evidence.
Intermediate association/pasting traces and bounded replay receipts remain in
`action_trace_commitment`, outside the quotient key. `FinalActionKey_c` denotes
the quotient-key output of `CanonicalActionResult_c`; it is never embedded in
the theorem-subject DAG. A later bounded executable replay may abort, but that
failure mints neither an action image nor a law fact.

A local reindexing keeps the verified history `H` fixed.  There are no
duplicate caller carrier arguments. `Payload_c(Env)` pattern-projects the
candidate's `Built_c` record or the stored action-image payload and has exactly
one of these tagged shapes:

| `c` | Exact current action payload |
| --- | --- |
| `Formation` | `(Gamma,A,A_raw)` |
| `Abstraction` | `(Gamma,A,B,Xi_raw,cut,B_raw)` |
| `Aggregation` | `(Gamma,A,B,C_raw)` |
| `Transport` | `(Delta,Gamma,theta,A,C_raw)` |
| `Comparison` | `(Gamma,A,B,q,A_raw,B_raw)` |
| `DemandCompiler` | `(Gamma,G=(E,O),S,a,L,a_raw)` |
| `DischargeTransformer` | `(Delta,Gamma,theta,G=(E,O),S,a,L,d,d_raw)` |

For Formation, Abstraction, Aggregation, Comparison, and DemandCompiler,
`ActArg_c(Env)` consists exactly of a well-typed `theta:Delta->Gamma`, where
`Gamma` is projected from that payload. The one-step sequential endpoint
`ReindexPayload_c(theta,Payload_c(Env))`, retained only in `ActionTrace`, is
fixed by the corresponding row:

```text
Formation:
  (Delta,A[theta],A_raw[theta])
Abstraction:
  (Delta,A[theta],B[theta^A],
   ReCtx_raw(theta,A,Xi_raw),cut'=|Delta|,B_raw[theta^A])
Aggregation:
  (Delta,A[theta],B[theta^A],C_raw[theta])
Comparison:
  (Delta,A[theta],B[theta],q(A[theta],B[theta]),
   A_raw[theta],B_raw[theta])
DemandCompiler:
  (Delta,G[theta],S[theta],a[theta^E],Spec(S[theta],a[theta^E]),
   a_raw[theta^E]).
```

Every bracketed raw action is the unique A2 capture-safe recipe and retains
its complete lineage. `ReCtx_raw(theta,A,Xi_raw)` is the independently rebuilt
raw presentation of `Delta.A[theta]` obtained from the PublicContext action and
the iterated lift; it is not a substitution of a context as a term. The source
`cut=|Gamma|` remains only in lineage/action trace, while the typed final
payload uses the recomputed `cut'=|Delta|`.
`CanonicalActionResult_c(B,a')` applies this same row once to
`Payload_c(B)` using the single fixed right-associated composite encoded by
the complete flattened `a'`; it never folds the row over intermediate
payloads. Define, with no other arguments,

```text
Re_c(theta,Env)
  = let a' = ExtendActionNF(Env,theta);
    in ActionImage_c(Base(Env),a',ActionTrace(theta,Env)).
```

Thus `Re_c` is a dependent function on `(theta,Env)`: a domain/codomain
mismatch is ill typed, not an equality guard or a second source of values.
Each action independently rebuilds all dependent contexts, lifts, expected
types, output records, fields, evidence, and section-3 endpoint descriptors.

For Transport and DischargeTransformer, `ActArg_c(Env)` is instead a checked
context square over the `theta` projected from the current payload:

```text
Q = (rho:Delta'->Delta, sigma:Gamma'->Gamma,
     theta':Delta'->Gamma',
     eps: theta.rho approx_Sub sigma.theta').
```

`approx_Sub` is pointwise A2-O typed substitution equality. Define the exact
payload reindexings

```text
ReindexPayload_Transport(Q,Payload_Transport(Env))
  = (Delta',Gamma',theta',A[sigma],C_raw[rho]).

ReindexPayload_DischargeTransformer(
  Q,Payload_DischargeTransformer(Env))
  = (Delta',Gamma',theta',G[sigma],S[sigma],a[sigma^E],
     Spec(S[sigma],a[sigma^E]),d[sigma],d_raw[rho]).

CanonicalActionResult_c(B,a')
  applies the corresponding equation once to Payload_c(B) using the canonical
  composite square encoded by the complete a'.

Re_c(Q,Env)
  = let a' = ExtendSquareNF(Env,Q);
    in ActionImage_c(Base(Env),a',ActionTrace(Q,Env)).
```

Here the arbitrary-square naturality subjects are exactly

```text
Payload_Transport(Env)=(Delta,Gamma,theta,A,C_raw),
Payload_DischargeTransformer(Env)=
  (Delta,Gamma,theta,G=(E,O),S,a,L,d,d_raw),
```

followed by

```text
Nat_T(Q,Env):
  nf(C_raw[rho]) approx_ord nf((A[sigma])[theta'])

Nat_DT(Q,Env):
  nf(rho^*d_raw) approx_ord nf(theta'^*(sigma^*d)).
```

These displays define the step subject from the exact projected payload.  The
`CanonicalNat` stored in an action image applies the same display once to the
base payload and the complete canonical composite square in its normalized
action.

`Nat_T` retains the reindexed original raw-output comparison, the intermediate
`nf((A[theta])[rho])`, both association endpoints, and congruence under
`eps : theta.rho approx_Sub sigma.theta'`.  `Nat_DT` retains every reindexed
`G/S/a/L` index and contract, the intermediate discharges at
`theta.rho` and `sigma.theta'`, and the corresponding `eps`-congruence
endpoint.  These comparison types are derived from `eps`, but neither equality
is assumed or proved by A3-O.

For a context square, `FinalActionKey(Q,Env)` uses both vertical maps and the
retained `eps` subject to perform the same final-state recomputation.  It does
not discard nonprincipal lineage, action-introduced dependencies, footprints,
support evidence, dispositions, or theorem subjects.
The identity square is `(id_Delta,id_Gamma,theta,refl)`. If `Q1` has vertical
maps `(rho1,sigma1)` and `Q2` has `(rho2,sigma2)`, their composite has
`(rho1.rho2,sigma1.sigma2)` and the pointwise pasted equality. These formulas
fully define `Re_c` for every constructor.

For all seven schemas, A3-O freezes but does not prove the selector subjects:

```text
Eval^kappa_c(H,o,r)=Match(E) ->
  exists! q in U_(Birth(o)).
    origin_(nf(E))(root_c)={q} and q=o and sel_(ord,c)(nf(E))=q,
sel(nf(E)) = sel(E),
E approx_(env,c) E' -> sel(E)=sel(E'),
sel(rename_H(E)) = rename_H(sel(E)),
sel(Re_c(q,E)) = sel(E).
```

Here `q` is a substitution for the five single-base schemas and a checked
context square for Transport/DischargeTransformer. `rename_H` is an admissible
bijection of exact identifiers in the same history evidence, not local
substitution. No undefined action of a local substitution on a historical
occurrence is used.

Each constructor theorem record retains the exact envelope action subjects

```text
Re_c(id,E) approx_(env,c) E,
Re_c(q1.q2,E) approx_(env,c) Re_c(q2,Re_c(q1,E)),
```

where `q1.q2` is substitution composition or the context-square composition
defined above. It also retains `Nat_c`: for Formation, Abstraction,
Aggregation, Comparison, and DemandCompiler, these are exactly the displayed
`L_c approx_ord R_c` endpoints of section 3.  For Transport and
DischargeTransformer, `Nat_c` is respectively the arbitrary-square subject
`Nat_T(Q,Env)` or `Nat_DT(Q,Env)` above.  The three typed edges of each
`T0--T3` and `D0--D3` quartet are retained separately as the exact
whiskering/association coherence subjects. These are formal theorem subjects
only. Finite successful samples cannot prove them.

## 6. Nominal-erasure audit

Erasing dependent indices, hidden output indices, raw realization data, and
evidence yields exactly the frozen JG2a table:

| Constructor | Exact nominal erasure |
| --- | --- |
| `Formation` | `PublicContext -> PublicInterface` |
| `Abstraction` | `(PublicContext, PublicInterface) -> InterfaceFamily` |
| `Aggregation` | `InterfaceFamily -> PublicInterface` |
| `Transport` | `(Substitution, PublicInterface) -> PublicInterface` |
| `Comparison` | `(PublicInterface, PublicInterface) -> ComparisonWitness` |
| `DemandCompiler` | `(SealedPublicGrammar, DemandScheme) -> LiveDemand` |
| `DischargeTransformer` | `(Substitution, Discharge) -> Discharge` |

No eighth constructor or tenth sort has been introduced. `Aggregation` uses
only telescope concatenation; it was not selected to favor a later `Pi` or
`Sigma` result. `Comparison` is ordinary definitional comparison only and
cannot be cited as cubical binary trace.

## 7. Authority boundary and continuation

A3-O freezes definitions and theorem subjects. It does not mint:

- a Level-I or Level-II carrier member;
- a checked envelope or pair disposition;
- totality, disjointness, or an observed constructor assignment;
- an identity, composition, selector-stability, functor, or naturality proof;
- executable Rust or exact Rust/Agda correspondence;
- cubical comparison, support depth, recent factorization, or adequacy;
- GCap, quotient, weakening, marginal, `gamma`, provenance, bootstrap, or
  selective authority.

The ordered continuation is:

1. **JG2b2b2a-A4-R0 -- RC1 release audit -- DISCHARGED 2026-08-02.** The
   candidate bytes replayed, but its semantic and representation freeze did not.
2. **JG2b2b2a-A4-R1 -- opaque evaluator and noncircular source census --
   FROZEN 2026-08-02.** Independent audit passed.
3. **JG2b2b2a-A4-R2 -- complete typed schemas and dynamic codecs -- ACTIVE AT
   R2c.** R2a and R2b are frozen; R2c--R2d precede R3, then R2e joins the complete
   registry/source identity before R4 regenerates fixtures.
4. **JG2b2b2a-A2-C -- cubical indexed ontologies -- OPEN IN PARALLEL.** It
   cannot consume ordinary comparison as a cubical bridge.
5. **JG2b2b2b -- executable ordinary representation and particular verifier --
   BLOCKED BY A4-R4.** No code authority exists yet.

Full JG2b2b2a and all downstream JG2 gates remain open.
