# JG2b2b2 Indexed-Interface Ontology Audit

Date: 2026-08-02

Status: **JG2b2b2-P0 AUDIT DISCHARGED.** Full JG2b2b2 is not discharged and
no indexed-interface implementation exists yet. The active ordinary
representation frontier is **JG2b2b2a-A4-R2c**: R2a and R2b are frozen, while the A4-O RC1 profile transcript
was reproducible but failed its release audit. The still-open closure sits over
A0 source concordance, the A1 common envelope protocol, the A2-O
nine-sort carrier/action substrate and parametric proposal operator recorded
in `docs/260802_jg2b2b2a_indexed_ontology_profiles.md`, and the frozen A3-O
ordinary constructor realization/classifier specification recorded in
`docs/260802_jg2b2b2a_a3_ordinary_constructor_schemas.md` and the A4-O record in
`docs/260802_jg2b2b2a_a4_ordinary_profile_closure.md`, with the controlling
audit in `docs/260802_jg2b2b2a_a4_rc1_release_audit.md`. A2-O alone is not a
complete/selectable profile or a constructor-indexed candidate universe. The
cubical A2-C ontology lanes remain open in parallel.
JG2b2b2b executable implementation is blocked by A4-R4; JG2b2b3a generic substitution
proof remains an ordered downstream gate.

The supplied cubical depth-two sources add one correction to this P0 record:
the canonical univalent comparison cannot be reduced to `pen-kernel`
definitional equality, and exact higher structural obligations cannot be
discarded without witnessed open-box replacement and an adequacy theorem. The
current kernel remains an exact ordinary dependent-calculus anchor, not a
cubical realization by itself.

## 1. Audit question and result

The P0 question was whether the existing JG1/JG2a/JG2b2 foundations already
determine a concrete nine-sort indexed calculus strongly enough that the Rust
representation can be implemented without making new mathematical choices.

They do not. The repository freezes:

1. a closed ordered vocabulary of nine **interface sort names**;
2. a closed ordered vocabulary of seven **capability constructor names**;
3. seven pairwise-distinct nominal input/output signatures over those sort
   names;
4. a complete structural grammar for locating term occurrences;
5. a direction and executable verifier for one particular open typed
   substitution; and
6. a protocol requiring generic substitution, indexed functor laws, and seven
   constructor naturality equations before universal authority may exist.

It does not yet freeze the dependent indices of the nine sorts, their values,
well-formedness judgments, equality, normalization, variance, reindexing
actions, constructor telescopes, closed classifier schemas, or exact theorem
subjects. Those choices affect what the later generic theorem even says.
They are therefore mathematical premises, not Rust implementation details.

The P0 audit is discharged by registering that underdetermination and the
constraints on its lawful resolution. It mints no token, changes no authority
flag, and proves no interface, substitution, functor, classifier, or
naturality fact.

## 2. Exact inherited finite split

### 2.1 The nine ordered interface sorts

The existing JG2a vocabulary, in canonical order, is:

1. `PublicContext`;
2. `PublicInterface`;
3. `InterfaceFamily`;
4. `Substitution`;
5. `ComparisonWitness`;
6. `SealedPublicGrammar`;
7. `DemandScheme`;
8. `LiveDemand`; and
9. `Discharge`.

These are currently target-neutral **sort labels**. Their presence in the
closed enum proves neither that a member exists nor that a member has been
kernel checked. In particular, the enum does not say which context indexes an
interface, which two contexts index a substitution, which demand indexes a
discharge, or which endpoints a comparison witness compares.

### 2.2 The seven ordered constructors

The existing JG2a constructor vocabulary, its JG1 role binding, and its
nominal sort signature are exactly:

| Constructor | JG1 role | Existing nominal signature |
| --- | --- | --- |
| `Formation` | `Formation` | `PublicContext -> PublicInterface` |
| `Abstraction` | `Abstraction` | `(PublicContext, PublicInterface) -> InterfaceFamily` |
| `Aggregation` | `Aggregation` | `InterfaceFamily -> PublicInterface` |
| `Transport` | `Transport` | `(Substitution, PublicInterface) -> PublicInterface` |
| `Comparison` | `Comparison` | `(PublicInterface, PublicInterface) -> ComparisonWitness` |
| `DemandCompiler` | `Compiler` | `(SealedPublicGrammar, DemandScheme) -> LiveDemand` |
| `DischargeTransformer` | `DischargeTransformer` | `(Substitution, Discharge) -> Discharge` |

The split is intentionally **nine sorts versus seven constructors**. The nine
sorts are not nine constructors, and the seven constructors are not seven
ordinary `Term` constructors. The constructor table classifies kinds of
reusable generative operation. It does not supply their dependent typing
rules or implementations.

### 2.3 What the nominal signatures leave open

Each nominal arrow suppresses dependencies that are load-bearing for
substitution and naturality:

- `Formation` does not yet state how its result records that it is an
  interface over the supplied public context.
- `Abstraction` does not yet state the compatibility relation between its
  context and interface or the index of the resulting family.
- `Aggregation` does not yet state at which family index its output interface
  lives or what operation aggregates the family.
- `Transport` does not yet state the source/codomain compatibility condition,
  the substitution orientation, or the context of the transported output.
  JG2b2b1 fixes the convention `theta : Delta -> Gamma`, with images living in
  `Delta`; the indexed carrier still has to internalize that convention.
- `Comparison` does not yet require a common context, identify the compared
  endpoints, or distinguish definitional equality from a separate comparison
  relation.
- `DemandCompiler` does not yet state which sealed grammar a scheme ranges
  over, which public context or interface indexes the resulting demand, or
  what makes the compilation unique.
- `DischargeTransformer` does not yet state which live demand a discharge
  answers, how the substitution acts on that demand, or how the output
  discharge is indexed.

Consequently the current signatures cannot support a determinate classifier
schema: more
than one inequivalent dependent telescope can erase to the same nominal
input/output list. Choosing one while writing Rust would silently add a
premise after the intended target and naturality obligations are already
known.

## 3. Rejected apparent shortcuts

### 3.1 Nine nominal `OpenJudgment` wrappers

It is not sufficient to define nine newtype wrappers around
`pen_kernel::OpenJudgment`. `OpenJudgment` has three kernel forms:
`TypeFormation`, `HasType`, and `DefinitionallyEqual`. It can establish that a
particular term-level judgment checks in a supplied dependent context, but it
does not by itself provide:

- nine disjoint dependent carriers;
- sort-specific indices or well-formedness;
- source and target indices for substitutions;
- endpoint indices for comparisons;
- grammar/scheme/demand/discharge dependency;
- sort-specific equality and normalization;
- variance and reindexing actions;
- constructor-specific classifier schemas; or
- the exact subjects of the later nine functor laws and seven naturality
  equations.

Nominal wrappers would therefore make a caller-selected wrapper tag do the
work of a derived classifier. They would also permit two differently indexed
objects with the same underlying kernel judgment to collapse accidentally.
`OpenJudgment` remains appropriate at the realization/evidence level; it is
not the missing ontology.

### 3.2 Fitting the ontology to desired `Pi`/`Sigma` outcomes

The contextual-internalization frontier predicts dependent-product and
dependent-sum structure as held-out evidence. Defining `InterfaceFamily`,
`Abstraction`, `Aggregation`, `DemandScheme`, or `Discharge` specifically so
that the later live process produces `Pi` or `Sigma` dischargers would be
target fitting and would void the experiment.

This rejection does **not** ban `Term::Pi` or `Term::Sigma` from the kernel
realization layer. They are already ordinary members of the independently
frozen twelve-form term grammar and must be handled uniformly with the other
forms. What is forbidden is selecting the new indexed ontology, constructor
telescopes, classifiers, or equality rules because they favor the predicted
future structures.

### 3.3 Importing `pen-law` to obtain ready-made domain objects

`pen-law` contains downstream Law V2 history, profile, demand, bootstrap, and
selection-facing contracts. Importing it into the isolated generative audit
to supply meanings for `DemandScheme`, `LiveDemand`, `Discharge`, or any other
sort would reverse the intended dependency and contaminate the target-neutral
pre-exposure boundary. It could also make a registered history/profile type or
an outcome-bearing contract masquerade as generative authority.

JG2b2b2 must remain defined inside the isolated `pen-generative-audit`
workspace from its reviewed parents and `pen-kernel` foundations. A later
downstream adapter may relate an independently certified generative object to
a Law V2 orchestration type. Such an adapter is not part of the ontology and
cannot be used to choose it.

## 4. Required two-level target-neutral calculus

JG2b2b2a must freeze a calculus with two explicit levels. Neither level may be
identified with the other by convention.

### 4.1 Level I: indexed generative objects

The ontology level must give a closed algebraic definition of each of the nine
sorts, including:

1. its complete ordered index telescope;
2. its canonical values or value grammar;
3. its formation and well-formedness judgments;
4. its canonical encoding and structural identity;
5. its equality and normalization relation;
6. its variance in every context-like index;
7. the domain and codomain of every reindexing action; and
8. the distinction between data, propositions, and evidence.

All cross-sort dependencies must be represented structurally. At minimum, the
freeze must make source/codomain contexts part of a substitution's index,
make an interface's public context recoverable without a caller assertion,
make a comparison witness bind its exact endpoints, and make every live demand
and discharge bind the grammar/scheme/demand chain to which it belongs.

The freeze must decide, rather than leave to implementation, whether an action
is covariant, contravariant, or two-sided. The bi-indexed `Substitution`
carrier must expose both endpoint actions needed by the later two-sided
whiskering laws. No endpoint, variance bit, or classifier tag may be supplied
as an unchecked Boolean or enum field when it can be derived from retained
parents.

### 4.2 Level II: checked realizations and replay evidence

The realization level must state how a Level-I object is represented and
checked. Every ordinary dependent component must bind the exact `pen-kernel`
dependent context, term, and open-judgment calculus. A profile that contains
paths, equivalences, transport, composition, filling, or univalence must also
bind a separately specified checked cubical realization and a preservation/
reflection bridge; encoding those operations as opaque kernel globals does
not establish their semantics. The realization level must:

- retain the full Level-I object and all relevant indices;
- retain the complete proposed kernel representation, not only its digest;
- derive every required `OpenJudgment` from the object's constructor and
  indices rather than accept caller-authored expected judgments;
- batch related judgments under one reviewed aggregate resource profile;
- retain normalized outputs and the exact replay evidence;
- distinguish structural identity, definitional equality, and any
  sort-specific comparison relation;
- use the JG2b2b1 orientation and capture-safe particular reindexing substrate;
  and
- reject any representation whose normalized replay no longer binds the same
  Level-I indices.

In particular, `ComparisonWitness` in a profile claiming the canonical
depth-two theorem must retain its endpoints, exported path or equivalence, and
transport/naturality square. Kernel normal-form equality may realize a
restricted ordinary profile or a reflexive component, but it is not the
univalent binary lower bound. Exact arity-three-and-higher factors remain in
the representation unless a full derivedness witness and applicable adequacy
theorem justify replacement.

Not every Level-I value must be encoded as one kernel term. Structured values
such as sealed grammars, schemes, live demands, or discharges may require
closed target-neutral records with embedded kernel-realized components. The
freeze must state this representation explicitly; a digest, name, or nominal
`OpenJudgment` wrapper is not a realization.

### 4.3 Constructor telescopes, classifiers, and theorem subjects

For each of the seven constructors, JG2b2b2a must freeze:

1. the complete dependent input telescope;
2. all compatibility premises derivable from earlier inputs;
3. the indexed output family;
4. the kernel realization obligations;
5. one deterministic typed predicate schema returning match or non-match for
   that constructor, with verifier/resource failure issuing no disposition;
6. derivation of the constructor key from the schema being evaluated rather
   than from a caller constructor tag;
7. the exact reindexing square, including both endpoints; and
8. the canonical theorem subject consumed later by JG2b2b3c.

Exactly one schema per constructor means closed vocabulary coverage, not that
the seven predicates are jointly total or mutually exclusive on occurrences.
The later JG2b2c aggregate deliberately distinguishes a unique match, zero
matches, and typed ambiguity after every occurrence/constructor pair has a
derived disposition.

Erasing each dependent telescope to nominal sort names must reproduce the
seven frozen JG2a signatures exactly. The dependent rules may refine that
grammar; they may not add an eighth constructor, an extensible `Other`, or a
new nominal sort. Conversely, exact nominal agreement is necessary but not
sufficient: the dependencies, realization judgments, and theorem subject are
part of the freeze.

For each of the nine sorts, the freeze must also state the exact identity and
composition propositions later proved in JG2b2b3b. JG2b2b2a specifies those
propositions; it does not prove them.

## 5. Parent and dependency boundary

The ontology/representation freeze must bind the following existing opaque
parents by their exact canonical authorities, not accept caller-supplied
digest stand-ins:

- `VerifiedPreExposureGenerativeCapabilityGrammarV1` for the seven JG1 roles
  and admission boundary;
- `VerifiedGenerativeCapabilityConstructorGrammarV1` for the exact nine-sort,
  seven-constructor nominal split;
- `VerifiedGenerativeSubstitutionNaturalityProtocolV1` for the repaired
  universal-theorem obligations;
- `VerifiedGenerativeStructuralOccurrenceGrammarV1` for the closed term/root/
  edge/binder vocabulary used by later classifiers and theorem subjects; and
- `VerifiedParticularOpenTypedSubstitutionProtocolV1` for the exact particular
  substitution orientation and replay discipline.

JG2b2b2a takes no complete history, occurrence list, stage slice, selected
constructor, live profile, semantic-family receipt, `nu`, `gamma`, held-out
label, or `pen-law` object. It does not consume a caller-minted
`VerifiedParticularOpenTypedSubstitutionV1` as evidence of a law. Concrete
particular instances needed by a later factual verifier must be derived
internally from the frozen representation and rechecked at that time.

Binding a parent means retaining enough exact evidence to prevent replay under
a different grammar or resource policy. A digest remains an identifier, not
proof that the identified parent was verified.

## 6. Authority boundary

The phases have deliberately different authority:

- This P0 audit records the gap and constraints only. It mints nothing.
- JG2b2b2a may mint only remintable ontology/representation-grammar authority:
  the statement that one exact closed definition matches the freeze. It mints
  no interface member, classifier result, substitution law, functor law, or
  naturality equation.
- JG2b2b2b may mint executable **particular representation** evidence only
  after deriving and replaying the complete obligations for that value or
  constructor application. It must not promote any finite collection of
  successes to generic authority.
- JG2b2b3a alone is responsible for the generic substitution metatheory and
  exact Rust/safe-Agda correspondence.
- JG2b2b3b remains responsible for the nine indexed identity/composition laws,
  including two-sided substitution whiskering.
- JG2b2b3c remains responsible for all seven constructor naturality equations
  and for minting combined universal authority only from the complete prior
  package.

Accordingly `executable_indexed_interface_authority`,
`indexed_interface_functor_law_authority`, and
`universal_naturality_authority` remain false at the end of this audit.

## 7. Resource and evidence boundary

The executable successor must inherit the fail-closed discipline already used
by JG2b2b1:

- preflight untrusted recursive values before clone, equality, encoding, or
  hashing;
- publish fixed reviewed operation, depth, material, and normalization-fuel
  ceilings rather than accept theorem-domain-changing caller caps;
- charge a complete constructor application or related judgment family to one
  aggregate budget rather than reset a fresh budget per field;
- derive all expected judgments, normalized endpoints, and classifiers
  internally;
- retain full parents, raw proposals, normalized results, and replay evidence;
  and
- treat exhaustion as `Unknown`/no evidence, never as a negative theorem and
  never as permission to certify a completed prefix.

Those operational ceilings constrain the Rust verifier, not the mathematical
domain of the later generic proof. JG2b2b3a must quantify over unbounded formal
well-typed derivations. Its exact correspondence theorem may state that a
bounded Rust success reflects the formal calculus; it may not claim that the
fixed Rust resource profile is closed under all composition or lifting.

## 8. JG2b2b2a exit criteria

The active ontology/representation freeze is discharged only when one phase
record and one closed canonical definition specify, without implementation-
time choices:

1. all nine dependent sort telescopes and value grammars;
2. all formation and well-formedness rules;
3. equality, normalization, and variance for every sort;
4. every reindexing action and endpoint orientation;
5. both actions of the bi-indexed substitution carrier;
6. all seven dependent constructor telescopes and realization obligations;
7. one deterministic derived classifier schema per constructor, with no
   caller constructor tag and no claim that the seven predicates are jointly
   total or mutually exclusive;
8. the nine exact functor-law subjects and seven exact naturality-square
   subjects;
9. canonical encodings, versioning, parent bindings, and mutation-sensitive
   protocol transcript;
10. the Level-I/Level-II realization relation;
11. the fixed resource policy and evidence-retention contract; and
12. an explicit non-authority list showing that no theorem or factual
    occurrence has been minted.

If the registered principles do not uniquely determine one ontology, the
freeze must record the remaining alternatives and either derive a principled
selection rule or register separate pre-exposure profiles. It may not choose
the alternative that happens to yield the expected `Pi`/`Sigma` continuation.

## 9. Ordered continuation

The lawful continuation is now:

1. **JG2b2b2-P0 -- ontology audit -- DISCHARGED 2026-08-02.** Register the
   exact nine-sort/seven-constructor gap, rejected shortcuts, and freeze
   conditions. No implementation or authority token.
2. **JG2b2b2a -- two-level indexed ontology and representation freeze --
   ORDINARY LANE FROZEN THROUGH A4-R2b; A4-R2c ACTIVE; A2-C OPEN.** A0 has discharged source concordance and the cubical-
   adequacy correction; A1 has frozen the common proof-carrying envelope;
   A2-O has frozen the ordinary nine-sort carrier/action substrate and
   constructor-parametric finite proposal operator; A3-O has frozen the seven
   ordinary constructor realization/classifier schemas and exact later theorem
   subjects. A4-O RC1 was rejected; `R2c -> R2d -> R3 -> R2e -> R4` must repair the canonical profile
   transcript, parent/resource contract, ambiguity/non-match coverage, and
   non-authority boundary. The two cubical claim contracts
   remain open at A2-C and cannot feed ordinary downstream authority. Only the
   applicable complete package discharges its profile lane.
3. **JG2b2b2b -- executable Rust representation and particular verifier --
   BLOCKED BY A4-R4.** Implement exactly the independently frozen repaired
   ordinary calculus, derive all
   obligations and classifiers internally, pin adversarial mutations, and
   mint only particular executable evidence.
4. **JG2b2b3a -- generic substitution proof and exact Rust/safe-Agda
   correspondence -- BLOCKED BY JG2b2b2b.** Prove the generic substitution,
   typing, typed-equality, lifting, and normalization/reindexing laws over the
   frozen calculus and bind them to the exact Rust representation.
5. **JG2b2b3b -- nine functor laws -- BLOCKED.** Prove identity and
   composition for every frozen action.
6. **JG2b2b3c -- seven constructor squares and combined universal authority --
   BLOCKED.** Prove naturality for all seven constructors and combine authority
   only after every prerequisite agrees.
7. **JG2b2c -- complete factual disposition matrix -- BLOCKED.** Only then may
   the history-consuming census derive concrete occurrences and theorem
   applications internally.

Full JG2b2, full JG2b, and full JG2 remain open. The ordinary representation
frontier is A4-R2c envelope/evidence/support/subject schema closure, with JG2b2b2b blocked by A4-R4; the
cubical specification frontier is A2-C. Neither is
theorem authority or a live generative-capacity run.
